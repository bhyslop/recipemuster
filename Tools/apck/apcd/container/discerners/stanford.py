"""StanfordAIMI/stanford-deidentifier-base wrapper.

Loads the model once at import-time. The native taxonomy is flat 8-label
(O, VENDOR, DATE, HCW, HOSPITAL, ID, PATIENT, PHONE) — no BIO prefixes —
so consecutive identical non-O labels fuse into a single span.

Findings shape per APCPS Stanford finding schema:
    {"text", "start", "end", "label", "confidence"}
"""

import torch
from transformers import AutoTokenizer, AutoModelForTokenClassification


_MODEL_ID = "StanfordAIMI/stanford-deidentifier-base"

_tokenizer = AutoTokenizer.from_pretrained(_MODEL_ID)
_model = AutoModelForTokenClassification.from_pretrained(_MODEL_ID)
_model.eval()
_id2label = _model.config.id2label


def analyze(text):
    """Return a list of finding dicts for the given normalized text.

    Strategy:
      1. Tokenize with offset mapping so we can map subword tokens back to
         character spans in the original text.
      2. Run the model, take argmax + softmax confidence per token.
      3. Drop O-labeled tokens.
      4. Fuse consecutive tokens that share the same non-O label into one
         span. Confidence on the fused span is the mean of contributing
         token confidences.
    """
    if not text:
        return []

    enc = _tokenizer(
        text,
        return_offsets_mapping=True,
        return_tensors="pt",
        truncation=True,
        max_length=512,
    )
    offsets = enc.pop("offset_mapping")[0].tolist()

    with torch.no_grad():
        logits = _model(**enc).logits[0]
    probs = torch.softmax(logits, dim=-1)
    pred_ids = probs.argmax(dim=-1).tolist()
    pred_confs = probs.max(dim=-1).values.tolist()

    findings = []
    cur = None  # active span: {start, end, label, confs:[...]}

    for tok_idx, (start, end) in enumerate(offsets):
        # Special tokens (CLS/SEP/PAD) carry (0,0) offsets — skip them.
        if start == 0 and end == 0:
            continue

        label = _id2label[pred_ids[tok_idx]]
        conf = pred_confs[tok_idx]

        if label == "O":
            if cur is not None:
                findings.append(_finalize(cur, text))
                cur = None
            continue

        if cur is None or cur["label"] != label or start > cur["end"] + 1:
            if cur is not None:
                findings.append(_finalize(cur, text))
            cur = {"start": start, "end": end, "label": label, "confs": [conf]}
        else:
            cur["end"] = end
            cur["confs"].append(conf)

    if cur is not None:
        findings.append(_finalize(cur, text))

    return findings


def _finalize(span, text):
    return {
        "text": text[span["start"]:span["end"]],
        "start": span["start"],
        "end": span["end"],
        "label": span["label"],
        "confidence": sum(span["confs"]) / len(span["confs"]),
    }

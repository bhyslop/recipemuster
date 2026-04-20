"""scispaCy en_core_sci_md wrapper.

Emits two finding shapes (per APCPS spaCy schema), distinguished by `kind`:
    {"kind": "token",  "text", "start", "end", "pos", "tag", "morph", "lemma", "head", "dep"}
    {"kind": "entity", "text", "start", "end", "label"}
"""

import spacy


_nlp = spacy.load("en_core_sci_md")


def analyze(text):
    if not text:
        return []

    doc = _nlp(text)
    findings = []

    for tok in doc:
        if tok.is_space:
            continue
        findings.append({
            "kind": "token",
            "text": tok.text,
            "start": tok.idx,
            "end": tok.idx + len(tok.text),
            "pos": tok.pos_,
            "tag": tok.tag_,
            "morph": str(tok.morph),
            "lemma": tok.lemma_,
            "head": tok.head.i,
            "dep": tok.dep_,
        })

    for ent in doc.ents:
        findings.append({
            "kind": "entity",
            "text": ent.text,
            "start": ent.start_char,
            "end": ent.end_char,
            "label": ent.label_,
        })

    return findings

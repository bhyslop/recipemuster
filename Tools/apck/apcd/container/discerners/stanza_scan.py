"""Stanza English UD pipeline wrapper.

Per A9AAV guidance: emit all available Stanza outputs at the token level
(upos, xpos, feats, lemma, head, deprel, BIO ner tag) plus aggregated
entity spans. Combining can decide later which signals matter.

Findings shape per APCPS Stanza schema:
    {"kind": "token",  "text", "start", "end", "upos", "xpos", "feats", "lemma", "head", "deprel", "ner"}
    {"kind": "entity", "text", "start", "end", "label"}
"""

import stanza


_pipeline = stanza.Pipeline(
    lang="en",
    dir="/opt/models/stanza",
    processors="tokenize,pos,lemma,depparse,ner",
    download_method=stanza.DownloadMethod.REUSE_RESOURCES,
    use_gpu=False,
    verbose=False,
)


def analyze(text):
    if not text:
        return []

    doc = _pipeline(text)
    findings = []

    for sent in doc.sentences:
        for word in sent.words:
            # word.parent is the Token containing this Word; for non-MWT
            # English data, word and parent line up 1:1.
            parent = word.parent
            findings.append({
                "kind": "token",
                "text": word.text,
                "start": parent.start_char,
                "end": parent.end_char,
                "upos": word.upos or "",
                "xpos": word.xpos or "",
                "feats": word.feats or "",
                "lemma": word.lemma or "",
                "head": word.head,
                "deprel": word.deprel or "",
                "ner": parent.ner or "O",
            })

    for ent in doc.entities:
        findings.append({
            "kind": "entity",
            "text": ent.text,
            "start": ent.start_char,
            "end": ent.end_char,
            "label": ent.type,
        })

    return findings

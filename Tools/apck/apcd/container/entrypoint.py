"""APCK container entrypoint.

Long-running loop:
  - Scan ${APCS_BINDMOUNT} for {N}-in.txt files.
  - Pick the highest N. Skip if {N}.json already exists, or if N is below
    the last-processed index (drop-old backlog policy).
  - Run all three discerners on the input text.
  - Write {N}.json.tmp, then atomic rename to {N}.json.
  - Append structured log lines to ${APCS_BINDMOUNT}/container-log.txt.

A failing discerner emits an empty findings list and a logged error; the
other discerners still publish their findings.
"""

import datetime
import json
import os
import re
import sys
import time
import traceback


_BINDMOUNT = os.environ.get("APCS_BINDMOUNT", "/work/apcjd")
_LOG_PATH = os.path.join(_BINDMOUNT, "container-log.txt")
_POLL_SEC = 0.2

_INPUT_RE = re.compile(r"^(\d+)-in\.txt$")


def log(msg):
    line = "[{ts}] {msg}\n".format(
        ts=datetime.datetime.now().isoformat(timespec="milliseconds"),
        msg=msg,
    )
    sys.stdout.write(line)
    sys.stdout.flush()
    try:
        with open(_LOG_PATH, "a", encoding="utf-8") as f:
            f.write(line)
    except OSError as e:
        sys.stdout.write("log-tee-failed: {}\n".format(e))
        sys.stdout.flush()


def scan_highest_index():
    """Return the highest N for which {N}-in.txt exists, or None."""
    try:
        names = os.listdir(_BINDMOUNT)
    except FileNotFoundError:
        return None

    highest = None
    for name in names:
        m = _INPUT_RE.match(name)
        if not m:
            continue
        n = int(m.group(1))
        if highest is None or n > highest:
            highest = n
    return highest


def safe_analyze(name, module, text):
    """Run a discerner; on any exception, log + return empty findings."""
    try:
        findings = module.analyze(text)
        return findings, None
    except Exception:
        return [], traceback.format_exc()


def process_index(n):
    in_path = os.path.join(_BINDMOUNT, "{}-in.txt".format(n))
    out_tmp = os.path.join(_BINDMOUNT, "{}.json.tmp".format(n))
    out_final = os.path.join(_BINDMOUNT, "{}.json".format(n))

    try:
        with open(in_path, "r", encoding="utf-8") as f:
            text = f.read()
    except OSError as e:
        log("read-failed index={} err={}".format(n, e))
        return False

    payload = {"index": n}
    counts = {}
    for key, module in (("stanford", _stanford),
                        ("spacy", _spacy),
                        ("stanza", _stanza)):
        findings, err = safe_analyze(key, module, text)
        payload[key] = {"findings": findings}
        counts[key] = len(findings)
        if err is not None:
            log("discerner-failed key={} index={}\n{}".format(key, n, err))

    try:
        with open(out_tmp, "w", encoding="utf-8") as f:
            json.dump(payload, f, ensure_ascii=False)
        os.rename(out_tmp, out_final)
    except OSError as e:
        log("write-failed index={} err={}".format(n, e))
        return False

    log("processed index={} stanford={} spacy={} stanza={}".format(
        n, counts["stanford"], counts["spacy"], counts["stanza"],
    ))
    return True


def main():
    log("starting bindmount={}".format(_BINDMOUNT))

    # Discerner imports happen here, after the log path is known, so model
    # load progress is captured. Failures during load are fatal — we cannot
    # publish meaningful output without at least one discerner.
    global _stanford, _spacy, _stanza
    log("loading stanford")
    from discerners import stanford as _stanford
    log("loading spacy")
    from discerners import spacy_scan as _spacy
    log("loading stanza")
    from discerners import stanza_scan as _stanza
    log("ready")

    last_processed = -1
    while True:
        n = scan_highest_index()
        if n is None or n <= last_processed:
            time.sleep(_POLL_SEC)
            continue

        out_final = os.path.join(_BINDMOUNT, "{}.json".format(n))
        if os.path.exists(out_final):
            # Already processed by an earlier run; advance high-water mark
            # so we don't reconsider.
            last_processed = n
            time.sleep(_POLL_SEC)
            continue

        if process_index(n):
            last_processed = n
        time.sleep(_POLL_SEC)


if __name__ == "__main__":
    main()

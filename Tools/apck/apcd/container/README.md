<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# APCK Container

Long-running Python container that runs three NLP discerners
(`apcscs_stanford`, `apcscs_spacy`, `apcscs_stanza`) against
normalized clipboard text staged by Clipbuddy. See APCPS Container
Architecture for the conceptual contract.

The container is **decoupled from Clipbuddy**: it runs as a long-lived
process started by the developer (or, eventually, a system service on
Ann's machine), watches `$HOME/apcjd/` for `{N}-in.txt` files, and writes
consolidated `{N}.json` outputs back into the same directory.

## Lifecycle Tabtargets

| Tabtarget | Purpose |
|-----------|---------|
| `tt/apcw-cb.ContainerBuild.sh` | `docker build` the image (first build pulls ML models — 5–10 min) |
| `tt/apcw-cs.ContainerStart.sh` | Truncate `container-log.txt`, `docker run` with `--network=none --cap-drop=all --read-only` |
| `tt/apcw-cx.ContainerStop.sh` | `docker stop` + `docker rm` |
| `tt/apcw-ci.ContainerStatus.sh` | Container running state, image, bind-mount reachability, last log lines |

## Smoke Test

Verifies the wire contract end-to-end: image builds, container starts,
input file is consumed, all three discerners produce findings, output
file appears atomically.

1. **Build the image:**
   ```
   tt/apcw-cb.ContainerBuild.sh
   ```
   First run takes 5–10 min — torch CPU wheel + transformers + scispaCy
   model + Stanza English UD package are all pulled from upstream during
   the build so the runtime container can run with `--network=none`.

2. **Start the container:**
   ```
   tt/apcw-cs.ContainerStart.sh
   ```
   Container detaches; model load takes a further 30–90s before
   discerners are ready. Watch `~/apcjd/container-log.txt` for the
   `loading stanford → loading spacy → loading stanza → ready` sequence.

3. **Stage a normalized input:**
   ```
   printf 'Patient: Margaret Thornton. DOB: 03/15/1952. Seen at Mercy General by Dr. Chen.' \
     > ~/apcjd/10000-in.txt
   ```

4. **Wait for the output (up to 30s after `ready`):**
   ```
   for i in {1..30}; do
     test -f ~/apcjd/10000.json && break
     sleep 1
   done
   test -f ~/apcjd/10000.json && echo "OK" || echo "TIMEOUT"
   ```

5. **Validate the JSON shape:**
   ```
   python3 -c "
   import json
   d = json.load(open('$HOME/apcjd/10000.json'))
   assert d['index'] == 10000
   for k in ('stanford', 'spacy', 'stanza'):
       assert k in d, f'missing {k}'
       assert 'findings' in d[k], f'missing {k}.findings'
       assert isinstance(d[k]['findings'], list), f'{k}.findings not a list'
       print(f'{k}: {len(d[k][\"findings\"])} findings')
   "
   ```
   Expected: each discerner reports a non-zero finding count. Stanford
   should label `Margaret Thornton` as `PATIENT` and `Mercy General` as
   `HOSPITAL`.

6. **Verify no temp file remains:**
   ```
   test ! -f ~/apcjd/10000.json.tmp && echo "OK: no leftover .tmp"
   ```

7. **Verify the log is populated:**
   ```
   grep -E 'starting|ready|processed index=10000' ~/apcjd/container-log.txt
   ```

8. **Stop the container:**
   ```
   tt/apcw-cx.ContainerStop.sh
   ```

## Wire Format

See APCPS § Container Architecture / Wire Protocol and § Container
Output JSON Schema for the authoritative definition. Briefly:

- Inputs: `{N}-in.txt` — UTF-8 normalized text.
- Outputs: `{N}.json` — top-level `{"index": N, "stanford": {"findings": [...]}, "spacy": {...}, "stanza": {...}}`.
- Atomicity: the container writes `{N}.json.tmp` then `os.rename` to
  `{N}.json`. Readers see either the absent state or the final file.
- Highest-N policy: only the highest-indexed `{N}-in.txt` is processed;
  lower indices are ignored (drop-old backlog).

## Security Posture

Enforced by `apcw-cs`:

- `--network=none` — no network namespace; PHI cannot exfiltrate.
- `--cap-drop=all` — no Linux capabilities.
- `--read-only` — root filesystem is immutable; only the bind mount is writable.
- `--tmpfs /tmp` — Python needs a writable `/tmp` for some library transients.
- `--user nobody:nogroup` — non-root runtime.
- `-v $HOME/apcjd:/work/apcjd` — only the journal directory is bind-mounted.
- `-e APCS_BINDMOUNT=/work/apcjd` — entrypoint reads this to find its workspace.

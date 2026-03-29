#!/usr/bin/env python3
#
# Copyright 2026 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBTIR - Ifrit escape test runner
#
# Discovers and executes rbtie_* escape test modules from rbtid/.
# Designed to run inside the bottle container behind the sentry.
#
# Usage:
#   python3 rbtir_runner.py                  # run all modules
#   python3 rbtir_runner.py dns              # run modules matching category
#   python3 rbtir_runner.py dns_exfil        # run modules matching substring

import importlib.util
import json
import os
import sys
import time
import traceback
from pathlib import Path

RBTID_DIR = Path(__file__).parent
MODULE_PREFIX = "rbtie_"


def discover_modules(filter_pattern=None):
    """Find all rbtie_*.py modules in rbtid/, optionally filtered."""
    modules = sorted(RBTID_DIR.glob(f"{MODULE_PREFIX}*.py"))
    if filter_pattern:
        modules = [m for m in modules if filter_pattern in m.stem]
    return modules


def load_module(path):
    """Dynamically load a test module by file path."""
    spec = importlib.util.spec_from_file_location(path.stem, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def run_module(path):
    """Execute a single test module, return structured result."""
    result = {
        "module": path.stem,
        "category": extract_category(path.stem),
        "started_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    }

    try:
        mod = load_module(path)

        if not hasattr(mod, "run"):
            result["verdict"] = "ERROR"
            result["detail"] = "Module missing run() function"
            return result

        outcome = mod.run()

        if not isinstance(outcome, dict):
            result["verdict"] = "ERROR"
            result["detail"] = f"run() returned {type(outcome).__name__}, expected dict"
            return result

        result["verdict"] = outcome.get("verdict", "ERROR")
        result["detail"] = outcome.get("detail", "")
        result["assertions"] = outcome.get("assertions", [])

    except Exception:
        result["verdict"] = "ERROR"
        result["detail"] = traceback.format_exc()

    result["finished_at"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    return result


def extract_category(stem):
    """Extract category from module name: rbtie_dns_exfil → dns."""
    parts = stem.removeprefix(MODULE_PREFIX).split("_", 1)
    return parts[0] if parts else "unknown"


def print_result(result):
    """Print a single result in human-readable form."""
    verdict = result["verdict"]
    marker = {"PASS": "+", "FAIL": "!", "ERROR": "?", "SKIP": "-"}.get(verdict, "?")
    print(f"  [{marker}] {result['module']}: {verdict}")
    if result.get("detail") and verdict != "PASS":
        for line in str(result["detail"]).strip().splitlines():
            print(f"      {line}")
    for assertion in result.get("assertions", []):
        a_marker = "+" if assertion.get("passed") else "!"
        print(f"    [{a_marker}] {assertion.get('name', '?')}: {assertion.get('detail', '')}")


def main():
    filter_pattern = sys.argv[1] if len(sys.argv) > 1 else None
    modules = discover_modules(filter_pattern)

    if not modules:
        print(f"No escape test modules found" +
              (f" matching '{filter_pattern}'" if filter_pattern else ""))
        sys.exit(0)

    print(f"Ifrit escape test runner — {len(modules)} module(s)")
    print()

    results = []
    for path in modules:
        result = run_module(path)
        results.append(result)
        print_result(result)

    print()

    # Summary
    by_verdict = {}
    for r in results:
        v = r["verdict"]
        by_verdict[v] = by_verdict.get(v, 0) + 1

    parts = []
    for v in ["PASS", "FAIL", "ERROR", "SKIP"]:
        if v in by_verdict:
            parts.append(f"{by_verdict[v]} {v}")
    print(f"Summary: {', '.join(parts)}")

    # JSON output to file for programmatic consumption
    report_path = RBTID_DIR / "rbtir_last_run.json"
    with open(report_path, "w") as f:
        json.dump({"results": results}, f, indent=2)
    print(f"Report: {report_path}")

    # Exit code: 0 if all PASS or SKIP, 1 otherwise
    failing = by_verdict.get("FAIL", 0) + by_verdict.get("ERROR", 0)
    sys.exit(1 if failing else 0)


if __name__ == "__main__":
    main()

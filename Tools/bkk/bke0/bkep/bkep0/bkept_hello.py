# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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

"""The hello exemplar's suite: its interpreter is the one the project pins."""

import platform
import sys
from pathlib import Path

import pytest

from bkep0.bkepa_hello import bkepa_main

__all__ = [
    "bkept_the_greeting_names_this_interpreter",
    "bkept_the_interpreter_is_the_pinned_one",
]

ZBKEPT_PIN = Path(__file__).resolve().parent.parent / ".python-version"


def bkept_the_interpreter_is_the_pinned_one() -> None:
    """The running interpreter is the exact patch the project's pin names."""
    pinned = ZBKEPT_PIN.read_text().strip()
    print(f"interpreter {sys.executable} reports {platform.python_version()}, pin {pinned}")
    assert platform.python_version() == pinned


def bkept_the_greeting_names_this_interpreter(capsys: pytest.CaptureFixture[str]) -> None:
    """The app answers success and its greeting names this interpreter's version."""
    assert bkepa_main() == 0
    assert platform.python_version() in capsys.readouterr().out

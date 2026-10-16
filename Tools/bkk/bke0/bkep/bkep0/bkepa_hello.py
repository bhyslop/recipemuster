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

"""The hello exemplar's app: it greets, naming the interpreter it runs on."""

import platform
import sys

__all__ = ["bkepa_main"]


def bkepa_main() -> int:
    """Print a greeting naming this interpreter, and answer success."""
    print(f"hello from python {platform.python_version()} at {sys.executable}")
    return 0

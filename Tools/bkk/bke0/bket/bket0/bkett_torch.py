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

"""The torch exemplar's suite: it reports its build and best device, and computes there with hand-checked answers."""

import torch

__all__ = [
    "bkett_a_product_on_the_best_device_matches_its_hand_value",
    "bkett_an_fftn_round_trip_returns_its_input",
    "bkett_the_build_and_its_best_device_report",
]


def zbkett_best() -> torch.device:
    """The accelerator torch finds, or its CPU where it finds none — torch's own answer, never ours."""
    accelerator = torch.accelerator.current_accelerator()
    return accelerator if accelerator is not None else torch.device("cpu")


def bkett_the_build_and_its_best_device_report() -> None:
    """The build names itself and its best device, and a tensor lands on that device."""
    device = zbkett_best()
    print(f"torch {torch.__version__} (cuda {torch.version.cuda}) best device {device}")
    placed = torch.ones(1, device=device)
    assert placed.device.type == device.type


def bkett_a_product_on_the_best_device_matches_its_hand_value() -> None:
    """[[1, 2], [3, 4]] times [[5, 6], [7, 8]] is [[19, 22], [43, 50]]."""
    device = zbkett_best()
    left = torch.tensor([[1.0, 2.0], [3.0, 4.0]], device=device)
    right = torch.tensor([[5.0, 6.0], [7.0, 8.0]], device=device)
    assert torch.equal((left @ right).cpu(), torch.tensor([[19.0, 22.0], [43.0, 50.0]]))


def bkett_an_fftn_round_trip_returns_its_input() -> None:
    """An n-dimensional FFT followed by its inverse returns the input."""
    device = zbkett_best()
    given = torch.arange(24, dtype=torch.float32, device=device).reshape(2, 3, 4)
    returned = torch.fft.ifftn(torch.fft.fftn(given)).real
    assert torch.allclose(returned.cpu(), given.cpu(), atol=1e-4)

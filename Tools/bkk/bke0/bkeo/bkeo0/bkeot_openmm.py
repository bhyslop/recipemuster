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

"""The OpenMM exemplar's suite: its CPU platform stands, and two particles give a hand-checked energy."""

import openmm
import openmm.unit

__all__ = [
    "bkeot_the_cpu_platform_stands",
    "bkeot_two_particles_at_the_well_minimum_hold_minus_epsilon",
]

# A Lennard-Jones pair in OpenMM's default units, nanometres and kJ/mol.
ZBKEOT_SIGMA = 0.3
ZBKEOT_EPSILON = 0.5
ZBKEOT_MASS = 39.9
ZBKEOT_PLATFORM = "CPU"


def bkeot_the_cpu_platform_stands() -> None:
    """OpenMM's platform roster names its CPU platform."""
    names = [openmm.Platform.getPlatform(i).getName() for i in range(openmm.Platform.getNumPlatforms())]
    print(f"openmm {openmm.__version__} platforms: {names}")
    assert ZBKEOT_PLATFORM in names


def bkeot_two_particles_at_the_well_minimum_hold_minus_epsilon() -> None:
    """At r = 2^(1/6) sigma, 4 epsilon ((sigma/r)^12 - (sigma/r)^6) is exactly -epsilon."""
    system = openmm.System()
    force = openmm.NonbondedForce()
    force.setNonbondedMethod(openmm.NonbondedForce.NoCutoff)
    for _ in range(2):
        system.addParticle(ZBKEOT_MASS)
        force.addParticle(0.0, ZBKEOT_SIGMA, ZBKEOT_EPSILON)
    system.addForce(force)

    context = openmm.Context(
        system,
        openmm.VerletIntegrator(0.001),
        openmm.Platform.getPlatformByName(ZBKEOT_PLATFORM),
    )
    separation = 2 ** (1 / 6) * ZBKEOT_SIGMA
    context.setPositions([openmm.Vec3(0.0, 0.0, 0.0), openmm.Vec3(separation, 0.0, 0.0)])

    energy = context.getState(getEnergy=True).getPotentialEnergy().value_in_unit(openmm.unit.kilojoule_per_mole)
    print(f"energy at the well minimum: {energy} kJ/mol, hand value {-ZBKEOT_EPSILON}")
    assert abs(energy - -ZBKEOT_EPSILON) < 1e-6

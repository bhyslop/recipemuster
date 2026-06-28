# Simulation Reference Engines

*Salvaged from a ChatGPT conversation, 2026-06-28, before closing the ChatGPT
paid account. Content is AI-generated (ChatGPT) — leads to verify, not
authority.*

Context: a hobbyist interested in what a high-end GPU (e.g. RTX 5090) can do for
physics, looking for free/open-source engines to generate trustworthy **reference
solutions** (ground truth) to validate future GPU solver implementations.
Correctness first, speed second; must be inexpensive at hobbyist scope yet
complete.

## Game-engine physics is mostly CPU-bound

Unreal's Chaos physics runs **primarily on the CPU**. The GPU is used heavily for:

- Rendering
- Niagara particle effects
- Some cloth and destruction effects
- Experimental features

But the core stays CPU-driven:

- Rigid bodies
- Constraints / joints
- Collision solving
- Vehicle dynamics

Why mainstream game physics stays on CPU despite powerful GPUs:

- Physics has lots of branching and irregular memory access.
- Determinism is important.
- Many scenes don't have enough objects to saturate a GPU.

So Unreal's Chaos is **not** the showcase for "what can my GPU do for physics?"
For GPU-centric work, look instead to CUDA-native solvers, FEM codes, fluids,
particle methods, and EM solvers.

## Reference solvers by domain (free, correctness-first)

**Electromagnetics**
- ElmerFEM — excellent open-source FEM reference.
- OpenEMS — strong for RF / antennas.
- MEEP — gold-standard open-source FDTD.

**Structural / Mechanical FEM**
- CalculiX — probably the strongest free Abaqus-like reference.
- Code_Aster — extremely capable, steeper learning curve.

**Fluids**
- OpenFOAM — dominant open-source reference.

**Multiphysics**
- ElmerFEM
- MOOSE Framework

### Single best overall

For a hobbyist building GPU solvers and needing trustworthy test vectors:
**ElmerFEM** is probably the best overall choice — free, mature, handles EM /
thermal / structural / coupled problems, produces high-quality reference
results, can run very slowly if needed for accuracy, and requires no six-figure
commercial license.

If the focus narrows to **EM** specifically, seriously consider **MEEP** (wave
optics / photonics) and **OpenEMS** (RF / antenna) alongside Elmer. Together
those three cover a huge fraction of interesting electromagnetic problems.

## Kinematics

For pure kinematics (motion, joints, linkages, robot arms, mechanisms) you often
don't need FEM at all. Good reference engines:

- **MuJoCo** — arguably the best open-source reference for articulated
  rigid-body dynamics and kinematics.
- **Bullet Physics** — solid, widely studied.
- **DART** — robotics-focused.
- **Pinocchio** — extremely strong for forward / inverse kinematics and
  dynamics.
- **Drake** — very high-quality robotics and control toolkit.

By goal:

- Robot arms → Pinocchio or Drake.
- General articulated mechanisms → MuJoCo.
- Vehicle suspensions, linkages, engines, gear trains → MuJoCo is a very strong
  starting point.

For generating ground truth against a future GPU implementation, ranked:

1. MuJoCo
2. Drake
3. Pinocchio
4. Bullet

MuJoCo is particularly attractive because it is mathematically rigorous, open
source, and designed by people obsessed with numerical correctness rather than
game performance.

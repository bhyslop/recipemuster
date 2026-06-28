# GPU Physics Acceleration — State of the Art

*Salvaged from a ChatGPT conversation, 2026-06-28, before closing the ChatGPT
paid account. Content is AI-generated (ChatGPT) — leads to verify, not
authority. Dates and version numbers below are especially worth checking.*

Context: a decade ago physics simulation was overwhelmingly CPU-bound. Two
questions — has NVIDIA GPU acceleration changed that, and is the open-source
Bullet Physics project (long latent) still alive, with peers worth knowing?

## Has GPU acceleration displaced CPU physics?

Yes, substantially — but unevenly by domain. CUDA (introduced 2006) matured into
a general-purpose parallel framework, letting developers offload compute-heavy
kernels (force calculations, matrix solves, particle interactions) to thousands
of GPU threads instead of running them serially on CPU cores. Across the 2010s
this unlocked order-of-magnitude speed-ups for many physics workloads.

Bottom line: a decade ago most physics codes ran strictly on CPUs; today both
real-time engines (games/graphics) and HPC solvers (molecular dynamics, CFD,
quantum-circuit simulation) routinely exploit NVIDIA GPUs, often at 10–100×,
enabling simulations previously impractical on CPU alone.

(Caveat from the companion memo `simulation-reference-engines.md`: mainstream
*game* physics — e.g. Unreal's Chaos — stays largely CPU-driven because of heavy
branching, irregular memory access, determinism needs, and scenes too small to
saturate a GPU. The GPU wins shown here are concentrated in HPC/scientific
solvers and in particle-based real-time engines, not general game rigid-body.)

### Real-time physics engines

- **NVIDIA PhysX (v5.x)** — originally built for Ageia's dedicated PPUs, fully
  ported to CUDA-enabled GeForce GPUs after NVIDIA acquired Ageia. Modern PhysX
  offloads collision detection, rigid-body dynamics, cloth, soft bodies, and
  particle-based fluids from the CPU to the GPU.
- **NVIDIA FleX & Flow** — FleX is a unified particle-based solver (fluids,
  cloth, ropes, solids) representing everything as constraints on particles, all
  compute on the GPU, designed for game-engine integration.

### High-performance molecular dynamics (MD)

- **NAMD** — early GPU offload gave ~5× on nonbonded interactions; the newer
  "GPU-resident" mode in NAMD 3.0 moves integration and constraint calculations
  onto the GPU as well, roughly doubling performance over the older offload
  scheme for small-to-medium systems.
- **GROMACS** — 2022+ releases feature heterogeneous MPI+CUDA parallelization
  with GPU-direct communication, scaling across multi-node A100 clusters for
  tens-to-hundreds× over CPU-only runs.

### Emerging / specialized toolkits

- **HOOMD-blue** — Python-driven particle simulation (MD + Monte Carlo),
  optimized end-to-end for GPUs, supports in-situ analysis and integration with
  ML frameworks (e.g. TensorFlow).

## Is Bullet Physics alive? Open-source engine vitality

Bullet Physics is still actively developed, and several peer open-source engines
are alive. (All version numbers and dates below are ChatGPT-reported — verify.)

- **Bullet Physics (bullet3 / PyBullet)** — actively developed; the `bullet3`
  repo sees regular commits (reportedly into early 2025), and PyBullet 3.2.5 was
  reportedly published April 2025. GPU support: experimental GPU demos exist
  (broadphase, soft bodies), but the **main solver remains CPU-based**.
- **Open Dynamics Engine (ODE)** — stable and widely used but largely dormant;
  last official release reportedly 0.16.2, July 2020. A lightweight option if you
  need a stable library and don't need active development.
- **Project Chrono** — multi-physics C++ framework (rigid, flexible bodies,
  granular, fluid-solid), optional GPU modules; core repo updated as recently as
  April 2025; active community, dozens of modules. Best fit for multi-physics /
  HPC-style sims.
- **MuJoCo** — high-performance dynamics for robotics, biomechanics, ML;
  open-source v3.2.7 reportedly January 2025; GPU-friendly via recent DeepMind
  updates.
- **DART (Dynamic Animation and Robotics Toolkit)** — robotics and computer
  animation, full access to dynamics internals; DART 6.15.0 reportedly November
  2024.
- **NVIDIA PhysX** — core SDK on GitHub since 2018; reportedly in March 2025
  NVIDIA open-sourced all the GPU kernels under BSD-3 (rigid bodies, fluids,
  deformables).

### Picking by use case

- General game-style engine → **Bullet** or **PhysX** (both open and maintained).
- Multi-physics / HPC sims → **Chrono**.
- Robotics / ML research → **MuJoCo** or **DART** (MuJoCo GPU-friendly).
- Lightweight, stable, minimal-development → **ODE**.

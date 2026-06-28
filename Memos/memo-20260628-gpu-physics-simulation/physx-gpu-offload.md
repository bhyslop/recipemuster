# NVIDIA PhysX GPU Offload

*Salvaged from a ChatGPT conversation, 2026-06-28, before closing the ChatGPT
paid account. Content is AI-generated (ChatGPT) — leads to verify, not
authority.*

Context: survey of GPU-accelerated physics solvers, then a focused dive on how
NVIDIA PhysX models GPU offload and what each phase of its simulation pipeline
does.

## Survey of GPU-accelerated physics solvers

- **NVIDIA PhysX** — real-time rigid-body, soft-body (cloth, rope), particle and
  volumetric fluid simulation, offloaded to CUDA-enabled GeForce GPUs since
  PhysX SDK 3.x.
- **NVIDIA FleX** — unified, particle-based solver (fluids, soft/rigid bodies,
  cloth, ropes) built on Position-Based Dynamics; fully CUDA-accelerated for
  real-time effects.
- **NVIDIA Flow** — grid-based gas/fluid solver (fire, smoke) integrated into
  the PhysX suite, leveraging GPU compute for interactive rates.
- **NVIDIA Blast** — GPU-accelerated destruction/fracturing module (APEX
  Destruction successor) for high-performance breakable geometry.
- **Bullet Physics (Bullet 3)** — open-source rigid/soft-body engine with an
  optional OpenCL backend for broadphase, collision and constraint solve on GPU.
- **Project Chrono GPU** — CUDA-based DEM module for large granular-material
  systems (monodisperse spheres) with full-history frictional contact.
- **DualSPHysics** — weakly-compressible SPH fluid solver with full CUDA GPU
  support, achieving orders-of-magnitude speedups over CPU.
- **Taichi** — Python-embedded language offering GPU-accelerated MPM/PIC/FLIP,
  PDEs and general-purpose compute; powers billion-particle MPM demos.

## PhysX GPU offload model

**Opt-in CUDA offload.** PhysX runs entirely on the CPU by default. GPU
acceleration is enabled at scene creation:

```cpp
PxSceneDesc desc;
desc.flags |= PxSceneFlag::eENABLE_GPU_DYNAMICS;
desc.broadPhaseType = PxBroadPhaseType::eGPU;
```

This switches on the CUDA-based implementations of broadphase, contact
generation, and constraint solve.

**Asynchronous task scheduling.** Behind the scenes a `CudaContextManager` owns
a `GpuDispatcher` that batches `GpuTask`s — memory copies and kernel launches —
onto one or more CUDA streams. On SM ≥ 2.0 it overlaps data transfers with
compute to maximize occupancy.

**Simulation pipeline on GPU.** Calling `PxScene::simulate()` enqueues, in
order:

1. Broadphase (AABB tree / filtering)
2. Contact generation (pairwise collision kernels)
3. Constraint solve (iterative Jacobi / Gauss-Seidel kernels)

Results stay resident in GPU buffers until fetched via `fetchResults()` or read
through the Direct GPU API.

**Modular CUDA libraries.** GPU code lives in a separate `PhysXGpu` module (e.g.
`PhysX3GPU_*.dll` in 3.x, or the `PhysXGpu` folder in the SDK distro). Building
without GPU support means these libraries need not be linked, keeping the
footprint minimal.

## Pipeline phases in detail

### 1. Broad-phase (AABB culling ⇒ potential pairs)

- **Function:** quickly reject non-intersecting actors by testing their
  axis-aligned bounding boxes (AABBs).
- **GPU algorithm:** uses SAP (Sort-and-Sweep) or ABP (Alternating Bitonic
  Partitioning), selected via `PxSceneDesc::broadPhaseType`. Sorts all min/max
  AABB endpoints with a radix sort or parallel scan (e.g. Thrust/CUB), then in
  one pass identifies overlapping intervals.
- **Data flow:** scene AABBs are uploaded into CUDA buffers; after sorting and
  interval scans the output is a compact list of candidate index-pairs in GPU
  memory.

### 2. Contact generation (narrow-phase)

- **Function:** for each candidate pair from broad-phase, compute exact
  collision contacts (points, normals, penetration depths).
- **GPU implementation:** launches one thread (or warp) per pair, invoking
  shape-specific kernels — sphere–sphere, box–convex, or a GJK/EPA routine for
  arbitrary convex meshes. Geometry data (vertices, planes, polyhedra) lives in
  device memory; support-function evaluations and linear algebra run fully on the
  GPU.
- **Output:** per-pair contact buffers (up to N contacts each) with contact
  point, normal vector, depth, and Jacobian entries, all resident on the device.

### 3. Constraint solve (impulse-based solver)

- **Function:** enforce contact and joint constraints to compute new velocities
  and transforms.
- **Pipeline:**
  - *Island formation* — GPU builds "islands" of interacting bodies (groups
    connected by contacts/joints).
  - *Constraint assembly* — from contacts and joints, generate Jacobian rows and
    right-hand terms.
  - *Iterative solve* — a Jacobi-style or Projected Gauss-Seidel loop (typically
    8–16 iterations); each constraint is processed in parallel, computing impulse
    corrections, accumulating via warp-level reductions, and updating body
    velocities.
  - *Integrate & fetch* — update body transforms in GPU buffers, then either copy
    back to CPU or expose via the Direct GPU API.
- **Performance:** massive parallelism across islands, but solver convergence and
  memory coalescing remain the tuning knobs.

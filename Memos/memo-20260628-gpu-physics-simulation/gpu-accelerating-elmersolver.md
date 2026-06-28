# GPU-Accelerating ElmerSolver

*Salvaged from a ChatGPT conversation, 2026-06-28, before closing the ChatGPT
paid account. Content is AI-generated (ChatGPT) — leads to verify, not
authority.*

Context: how to GPU-accelerate ElmerSolver (the FEM engine behind ElmerFEM).
The conversation walked from a menu of acceleration avenues, through a tangent on
MPI's relevance, to a decision to drop clustering and target a single beefy node's
GPUs directly.

## Avenues to GPU-accelerate ElmerSolver (lowest → highest effort)

**1. Offload element-level assembly**
- Hotspot: the double-loop over elements and integration points that builds local
  stiffness/mass matrices.
- Approach: annotate or rewrite those loops with OpenACC / OpenMP offload or CUDA
  Fortran kernels, so each element's element-matrix calculation runs on the GPU.

**2. GPU-accelerated sparse-matrix assembly & operations**
- Hotspot: gathering local element contributions into the global sparse matrix and
  applying boundary conditions.
- Approach: use a GPU-aware sparse library (cuSPARSE, ViennaCL), or switch Elmer's
  global-matrix backend to PETSc / Trilinos compiled with GPU support.

**3. Iterative linear solvers & preconditioners on the GPU**
- Hotspot: Krylov solves (CG / GMRES) and multigrid / preconditioning.
- Approach: hook into GPU-accelerated solvers in PETSc or hypre's GPU-enabled AMG;
  or call cuBLAS / cuSOLVER directly for dense blocks in block-structured problems.

**4. End-to-end workflow via a performance-portability layer**
- Approach: refactor core compute kernels into a Kokkos or RAJA abstraction, then
  compile once for CPU or GPU targets.

**5. Data-movement & MPI+GPU balance**
- Consideration: minimize host–device transfers by keeping all per-time-step data
  resident on the GPU; overlap MPI halo exchanges with GPU compute.

### Suggested prototyping path

1. Profile a representative solve (e.g. a 3D magneto-quasistatic run) and find where
   >80% of time is spent.
2. Pick one kernel (e.g. element stiffness assembly), extract it into a standalone
   test case.
3. Experiment with an OpenACC prototype (or a simple CUDA kernel).
4. Measure speedup, then gradually integrate GPU calls back into ElmerSolver.

## On MPI ("Message Passing Interface")

Yes — it's the same MPI from the mid-1990s. Despite its age it remains the de-facto
standard for distributed-memory parallelism in HPC:

- **Portability & ecosystem** — every cluster supports an implementation (Open MPI,
  MPICH, Intel MPI). Thousands of scientific codes use it and interoperate.
- **Modern features** — GPU-awareness (CUDA-aware MPI, GPUDirect), nonblocking
  collectives, one-sided remote-memory access, fault-tolerance extensions.
- **Rich toolchain** — mature profilers, debuggers, tuned libraries, documentation.

Alternatives, if moving beyond MPI:

- **PGAS models** (UPC, Coarray Fortran, Chapel, OpenSHMEM) — global address space
  with one-sided ops; simpler patterns but less maturity / multi-vendor tuning.
- **Task-based runtimes** (Charm++, Legion, HPX) — asynchronous task graphs and
  dynamic load balancing instead of bulk-synchronous SPMD; good performance but
  substantial refactoring.
- **Hybrid** — MPI between nodes + threads (OpenMP) or a GPU runtime within a node.
  This is what Elmer does today (MPI across ranks + OpenMP / GPU offload per rank).

## Decision: drop clustering, target a single node's GPUs

Given a powerful single node, MPI/clustering is a distraction — target the node's
GPUs directly. On a single node:

- **Offload element-loop assembly** — find the nested element → integration-point
  loops in ElmerSolver's Fortran; prototype with OpenACC directives
  (`!$acc parallel loop`) or CUDA Fortran kernels.
- **GPU-accelerated sparse kernels** — replace Elmer's global assembly + solve with
  PETSc / Trilinos built with CUDA support (they call cuSPARSE / cuSOLVER), or link
  cuSPARSE for assembly and cuSOLVER for factorization / CG directly.
- **Keep data on-GPU between time-steps** — allocate primary vectors and matrices in
  GPU memory once; update only changed entries each Newton / time step to avoid PCIe
  transfers. Unified / CUDA Managed Memory eases a first prototype.
- **Multi-GPU within the node** — with NVLink-connected GPUs, partition the mesh by
  GPU and use CUDA-aware MPI *locally only*, or use NCCL for halo exchanges. Even
  without MPI, Kokkos (CUDA backend) can target multiple GPUs in one address space.
- **Portability layer** — refactor hot kernels into Kokkos or RAJA, then switch
  CPU vs CUDA backend at compile time.

First steps: pick the first hotspot (likely element assembly), extract that loop
into a minimal test case, add OpenACC pragmas or a simple CUDA kernel.

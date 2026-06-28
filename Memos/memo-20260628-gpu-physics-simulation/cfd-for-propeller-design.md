# CFD for Propeller Design

*Salvaged from a ChatGPT conversation, 2026-06-28, before closing the ChatGPT
paid account. Content is AI-generated (ChatGPT) — leads to verify, not
authority.*

Context: starting from a tutorial article — *"Navier-Stokes Fluid Simulation
Explained with Godot"* (myzopotamia.dev) — the question was whether that approach
could serve as a basis for predicting actual fluid behavior to design propellers.

## Is the Godot Navier-Stokes tutorial a basis for propeller prediction?

**No — not for engineering-grade prediction.**

The Godot article is a strong *learning toy*, not a design tool. Its method is the
classic game-style "stable fluids" approximation:

- 2D grid
- density advection
- velocity diffusion / advection
- projection step
- Gauss-Seidel solver
- runs on CPU

The author explicitly frames it as for learning, not for performance or precision.

**Best use:** learn the numerical vocabulary (advection, diffusion, projection,
the pressure-Poisson solve).
**Bad use:** predicting propeller thrust or efficiency.

## What real propeller design actually requires

At minimum:

- **3D rotating geometry**, or an actuator-disk / actuator-line model
- a **turbulence model** — RANS or LES, not a toy stable-fluids solver
- **boundary layers, separation, tip vortices**
- **Reynolds number and Mach effects**
- **validation** against measured thrust / torque / wake test data

NASA's CFD verification-and-validation guidance is the relevant standard: verify
and validate before trusting CFD for any design decision.

## More serious starting points

- **OpenFOAM** with rotating-mesh approaches — **MRF** (Multiple Reference Frame)
  or **AMI** (Arbitrary Mesh Interface) — for resolved rotating geometry.
  OpenFOAM also has **actuator disk / actuator line** support, and propeller
  studies commonly use the MRF/AMI route.
- **Panel methods / BEMT** (Blade Element Momentum Theory) codes for a fast
  first-pass propeller design before committing to full CFD.

The progression: use the Godot tutorial to learn the concepts, then move to
BEMT/panel methods for first-pass sizing, then OpenFOAM (MRF/AMI or
actuator-disk) for higher-fidelity prediction — always validated against test
data before it informs a design decision.

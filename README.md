# frank-wolfe-vi

A generic Rust library for projection-free Frank-Wolfe methods on monotone variational inequalities, with a Lean specification companion.

## What it is
`frank-wolfe-vi` is a reusable optimization library for solving variational inequality problems over compact convex sets using Frank-Wolfe-style updates.

## Design goals
- generic enough for multiple domains
- reusable operator/set abstractions
- useful convergence diagnostics
- Rust implementation with Lean-aligned mathematical specification

## What it includes
- monotone operator trait
- compact convex set trait with linear minimization oracle
- Frank-Wolfe VI solver
- gap and residual diagnostics
- simplex and box example sets
- Lean specification mirroring the core Rust concepts

## Use cases
- constrained optimization
- equilibrium computation
- game-theoretic experimentation
- resource allocation and routing prototypes
- optimization research/education tooling
- machine learning problems with simplex or box constraints

## Machine learning-oriented examples
### 1. Mixture weights / ensemble weighting on a simplex
Use the simplex set when model weights must stay nonnegative and sum to 1.

Run:
```bash
cargo run --example ml_simplex
```

This is a natural fit for:
- ensemble weighting
- mixture models
- attention-style weight tuning in simplified settings
- constrained probability-vector optimization

### 2. Box-constrained coefficient tuning
Use the box set when parameters must stay within bounded ranges.

Run:
```bash
cargo run --example ml_box
```

This is a natural fit for:
- bounded coefficient optimization
- lightweight hyperparameter tuning prototypes
- constrained parameter updates for ML systems
- simple online optimization experiments

## Honest scope
This is not a full formal verification of floating-point Rust behavior.
The Lean side is intended to mirror the algorithmic structure and core mathematical properties closely.

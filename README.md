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

## Honest scope
This is not a full formal verification of floating-point Rust behavior.
The Lean side is intended to mirror the algorithmic structure and core mathematical properties closely.

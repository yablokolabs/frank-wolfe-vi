# Rust ↔ Lean Alignment

This document explains the intended 1:1 concept alignment between the Rust library and the Lean specification.

## Rust concepts mirrored in Lean
- `MonotoneOperator`
- `CompactConvexSet`
- `FrankWolfeGap`
- `IterationStats` / `IterationState`
- `SolverReport`
- the Frank-Wolfe step update
- dot/add/sub/scale vector helpers

## Important note
The alignment is at the algorithm/spec level.
It is **not** a machine-checked proof of exact floating-point equivalence with the Rust implementation.

## Closest 1:1 mappings
Rust:
- `MonotoneOperator::evaluate`
- `CompactConvexSet::linear_minimization_oracle`
- `FrankWolfeSolver::solve`
- `FrankWolfeGap { value }`

Lean:
- `MonotoneOperator.eval`
- `CompactConvexSet.linearMinimizationOracle`
- `step`
- `frankWolfeGap`

## Current gap
The Lean side mirrors the structure directly, but still contains proof placeholders and does not model floating-point numerical error.

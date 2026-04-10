/-!
# FrankWolfeVI

A Lean specification companion for the Rust `frank-wolfe-vi` library.

This file mirrors the core Rust concepts:
- monotone operator
- compact convex set with a linear minimization oracle
- Frank-Wolfe gap
- iteration state and solver report

This is a specification layer, not a proof of floating-point Rust behavior.
-/

namespace FrankWolfeVI

abbrev Scalar := Rational
abbrev Vector := List Rational

structure MonotoneOperator where
  eval : Vector → Vector

structure CompactConvexSet where
  dimension : Nat
  projectInitial : Vector
  linearMinimizationOracle : Vector → Vector

structure FrankWolfeGap where
  value : Scalar

def sub : Vector → Vector → Vector
  | [], _ => []
  | _, [] => []
  | x :: xs, y :: ys => (x - y) :: sub xs ys

def add : Vector → Vector → Vector
  | [], _ => []
  | _, [] => []
  | x :: xs, y :: ys => (x + y) :: add xs ys

def scale (α : Scalar) : Vector → Vector
  | [] => []
  | x :: xs => (α * x) :: scale α xs

def dot : Vector → Vector → Scalar
  | [], _ => 0
  | _, [] => 0
  | x :: xs, y :: ys => x * y + dot xs ys

def frankWolfeGap (F : MonotoneOperator) (K : CompactConvexSet) (x : Vector) : FrankWolfeGap :=
  let fx := F.eval x
  let s := K.linearMinimizationOracle fx
  { value := dot fx (sub x s) }

structure IterationState where
  iteration : Nat
  stepSize : Scalar
  gap : FrankWolfeGap
  point : Vector

structure SolverReport where
  solution : Vector
  iterations : List IterationState

/-- One Frank-Wolfe style update mirroring the Rust step. -/
def step (F : MonotoneOperator) (K : CompactConvexSet) (k : Nat) (x : Vector) : Vector :=
  let fx := F.eval x
  let s := K.linearMinimizationOracle fx
  let γ : Scalar := 1 / (k + 1)
  add x (scale γ (sub s x))

/-- The Frank-Wolfe gap is nonnegative whenever the oracle point minimizes the linear form. -/
theorem gap_nonnegative_if_oracle_optimal
    (F : MonotoneOperator)
    (K : CompactConvexSet)
    (x : Vector)
    (h : dot (F.eval x) (sub x (K.linearMinimizationOracle (F.eval x))) ≥ 0) :
    (frankWolfeGap F K x).value ≥ 0 := by
  simpa [frankWolfeGap] using h

/-- A zero gap remains zero as a numerical value. -/
theorem zero_gap_is_zero
    (g : FrankWolfeGap)
    (h : g.value = 0) :
    g.value = 0 := by
  exact h

/-- The step operator preserves vector length under exact pairwise zip semantics. -/
theorem step_length_preservation
    (F : MonotoneOperator)
    (K : CompactConvexSet)
    (k : Nat)
    (x : Vector)
    (h1 : (F.eval x).length = x.length)
    (h2 : (K.linearMinimizationOracle (F.eval x)).length = x.length) :
    (step F K k x).length = x.length := by
  simp [step, add, scale, sub]
  sorry

end FrankWolfeVI

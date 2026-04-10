pub mod diagnostics;
pub mod operator;
pub mod set;
pub mod solver;

pub use diagnostics::{FrankWolfeGap, IterationStats, SolverReport};
pub use operator::{MonotoneOperator, VectorOperator};
pub use set::{BoxSet, CompactConvexSet, Simplex};
pub use solver::{FrankWolfeConfig, FrankWolfeError, FrankWolfeSolver};

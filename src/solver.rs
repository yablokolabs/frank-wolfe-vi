use thiserror::Error;

use crate::diagnostics::{FrankWolfeGap, IterationStats, SolverReport};
use crate::operator::MonotoneOperator;
use crate::set::CompactConvexSet;

#[derive(Debug, Clone, PartialEq)]
pub struct FrankWolfeConfig {
    pub max_iters: usize,
    pub tolerance: f64,
}

impl Default for FrankWolfeConfig {
    fn default() -> Self {
        Self {
            max_iters: 100,
            tolerance: 1e-6,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum FrankWolfeError {
    #[error("dimension mismatch between operator and feasible set")]
    DimensionMismatch,
    #[error("invalid initial point dimension")]
    InvalidInitialPoint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrankWolfeSolver {
    config: FrankWolfeConfig,
}

impl FrankWolfeSolver {
    pub fn new(config: FrankWolfeConfig) -> Self {
        Self { config }
    }

    pub fn solve<O, S>(
        &self,
        operator: &O,
        feasible_set: &S,
    ) -> Result<SolverReport, FrankWolfeError>
    where
        O: MonotoneOperator,
        S: CompactConvexSet,
    {
        if operator.dimension() != feasible_set.dimension() {
            return Err(FrankWolfeError::DimensionMismatch);
        }

        let mut x = feasible_set.project_initial();
        if x.len() != operator.dimension() {
            return Err(FrankWolfeError::InvalidInitialPoint);
        }

        let mut iterations = Vec::new();
        for k in 0..self.config.max_iters {
            let fx = operator.evaluate(&x);
            let s = feasible_set.linear_minimization_oracle(&fx);
            let gap_value = dot(&fx, &sub(&x, &s));
            let gap = FrankWolfeGap { value: gap_value };
            let step = 1.0 / ((k + 1) as f64);
            iterations.push(IterationStats {
                iteration: k,
                step_size: step,
                gap: gap.clone(),
                point: x.clone(),
            });
            if gap_value <= self.config.tolerance {
                break;
            }
            let direction = sub(&s, &x);
            x = add(&x, &scale(&direction, step));
        }

        Ok(SolverReport {
            solution: x,
            iterations,
        })
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

fn sub(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}

fn scale(v: &[f64], alpha: f64) -> Vec<f64> {
    v.iter().map(|x| x * alpha).collect()
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::operator::VectorOperator;
    use crate::set::{BoxSet, Simplex};

    #[test]
    fn simplex_solver_runs() {
        let op = VectorOperator::new(3, |x| x.to_vec());
        let set = Simplex::new(3);
        let solver = FrankWolfeSolver::new(FrankWolfeConfig::default());
        let report = solver.solve(&op, &set).unwrap();
        assert!(!report.iterations.is_empty());
        let sum: f64 = report.solution.iter().sum();
        assert_relative_eq!(sum, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn box_solver_respects_dimension() {
        let op = VectorOperator::new(2, |x| x.to_vec());
        let set = BoxSet::new(vec![0.0, 0.0], vec![1.0, 1.0]);
        let solver = FrankWolfeSolver::new(FrankWolfeConfig::default());
        let report = solver.solve(&op, &set).unwrap();
        assert_eq!(report.solution.len(), 2);
    }
}

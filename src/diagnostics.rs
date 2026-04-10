use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrankWolfeGap {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IterationStats {
    pub iteration: usize,
    pub step_size: f64,
    pub gap: FrankWolfeGap,
    pub point: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolverReport {
    pub solution: Vec<f64>,
    pub iterations: Vec<IterationStats>,
}

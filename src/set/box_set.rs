use super::CompactConvexSet;

#[derive(Debug, Clone, PartialEq)]
pub struct BoxSet {
    lower: Vec<f64>,
    upper: Vec<f64>,
}

impl BoxSet {
    pub fn new(lower: Vec<f64>, upper: Vec<f64>) -> Self {
        Self { lower, upper }
    }
}

impl CompactConvexSet for BoxSet {
    fn dimension(&self) -> usize {
        self.lower.len()
    }

    fn project_initial(&self) -> Vec<f64> {
        self.lower
            .iter()
            .zip(&self.upper)
            .map(|(l, u)| (l + u) / 2.0)
            .collect()
    }

    fn linear_minimization_oracle(&self, gradient: &[f64]) -> Vec<f64> {
        self.lower
            .iter()
            .zip(&self.upper)
            .zip(gradient)
            .map(|((l, u), g)| if *g >= 0.0 { *l } else { *u })
            .collect()
    }
}

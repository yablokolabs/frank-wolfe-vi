use super::CompactConvexSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Simplex {
    dim: usize,
}

impl Simplex {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl CompactConvexSet for Simplex {
    fn dimension(&self) -> usize {
        self.dim
    }

    fn project_initial(&self) -> Vec<f64> {
        vec![1.0 / self.dim as f64; self.dim]
    }

    fn linear_minimization_oracle(&self, gradient: &[f64]) -> Vec<f64> {
        let mut out = vec![0.0; self.dim];
        if let Some((idx, _)) = gradient
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(core::cmp::Ordering::Equal))
        {
            out[idx] = 1.0;
        }
        out
    }
}

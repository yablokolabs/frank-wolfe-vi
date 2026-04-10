pub trait MonotoneOperator {
    fn dimension(&self) -> usize;
    fn evaluate(&self, x: &[f64]) -> Vec<f64>;
}

#[derive(Debug, Clone)]
pub struct VectorOperator<F>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    dim: usize,
    f: F,
}

impl<F> VectorOperator<F>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    pub fn new(dim: usize, f: F) -> Self {
        Self { dim, f }
    }
}

impl<F> MonotoneOperator for VectorOperator<F>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    fn dimension(&self) -> usize {
        self.dim
    }

    fn evaluate(&self, x: &[f64]) -> Vec<f64> {
        (self.f)(x)
    }
}

mod box_set;
mod simplex;

pub use box_set::BoxSet;
pub use simplex::Simplex;

pub trait CompactConvexSet {
    fn dimension(&self) -> usize;
    fn project_initial(&self) -> Vec<f64>;
    fn linear_minimization_oracle(&self, gradient: &[f64]) -> Vec<f64>;
}

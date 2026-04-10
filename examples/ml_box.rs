use frank_wolfe_vi::{BoxSet, FrankWolfeConfig, FrankWolfeSolver, VectorOperator};

fn main() {
    // Toy ML-flavored box-constrained example:
    // tune three bounded hyperparameters / coefficients with a projection-free update.
    let operator = VectorOperator::new(3, |x| {
        vec![1.2 * x[0] - 0.4, 0.8 * x[1] - 0.2, 1.5 * x[2] - 0.7]
    });

    let feasible_set = BoxSet::new(vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 1.0]);
    let solver = FrankWolfeSolver::new(FrankWolfeConfig {
        max_iters: 60,
        tolerance: 1e-8,
    });

    let report = solver.solve(&operator, &feasible_set).unwrap();
    println!("solution = {:?}", report.solution);
    println!("iterations = {}", report.iterations.len());
}

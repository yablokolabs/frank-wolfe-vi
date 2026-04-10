use frank_wolfe_vi::{BoxSet, FrankWolfeConfig, FrankWolfeSolver, VectorOperator};

fn main() {
    // Toy fintech example:
    // tune bounded execution/risk parameters such as spread width,
    // inventory penalty, and participation aggressiveness.
    let operator = VectorOperator::new(3, |x| {
        vec![1.1 * x[0] - 0.3, 0.9 * x[1] - 0.45, 1.3 * x[2] - 0.6]
    });

    let feasible_set = BoxSet::new(vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 1.0]);
    let solver = FrankWolfeSolver::new(FrankWolfeConfig {
        max_iters: 60,
        tolerance: 1e-8,
    });

    let report = solver.solve(&operator, &feasible_set).unwrap();
    println!("bounded_fintech_controls = {:?}", report.solution);
    println!("iterations = {}", report.iterations.len());
}

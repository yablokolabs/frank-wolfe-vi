use frank_wolfe_vi::{BoxSet, FrankWolfeConfig, FrankWolfeSolver, VectorOperator};

fn main() {
    // Toy game-domain example:
    // tune bounded economy/control parameters like spawn rate, reward multiplier,
    // and pricing pressure within safe limits.
    let operator = VectorOperator::new(3, |x| {
        vec![1.0 * x[0] - 0.25, 1.3 * x[1] - 0.55, 0.9 * x[2] - 0.35]
    });

    let feasible_set = BoxSet::new(vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 1.0]);
    let solver = FrankWolfeSolver::new(FrankWolfeConfig {
        max_iters: 60,
        tolerance: 1e-8,
    });

    let report = solver.solve(&operator, &feasible_set).unwrap();
    println!("bounded_controls = {:?}", report.solution);
    println!("iterations = {}", report.iterations.len());
}

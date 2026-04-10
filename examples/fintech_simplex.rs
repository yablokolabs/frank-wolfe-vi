use frank_wolfe_vi::{FrankWolfeConfig, FrankWolfeSolver, Simplex, VectorOperator};

fn main() {
    // Toy fintech example:
    // allocate portfolio weights across three strategies under simplex constraints.
    let operator = VectorOperator::new(3, |x| {
        vec![1.4 * x[0] + 0.3, 1.1 * x[1] + 0.2, 1.6 * x[2] + 0.1]
    });

    let feasible_set = Simplex::new(3);
    let solver = FrankWolfeSolver::new(FrankWolfeConfig {
        max_iters: 50,
        tolerance: 1e-8,
    });

    let report = solver.solve(&operator, &feasible_set).unwrap();
    println!("portfolio_weights = {:?}", report.solution);
    println!(
        "final_gap = {:?}",
        report.iterations.last().map(|it| it.gap.value)
    );
}

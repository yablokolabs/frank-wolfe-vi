use frank_wolfe_vi::{FrankWolfeConfig, FrankWolfeSolver, Simplex, VectorOperator};

fn main() {
    // Toy ML-flavored example:
    // optimize mixture weights over a simplex where the operator acts like
    // a gradient signal over model/component weights.
    let operator = VectorOperator::new(3, |x| {
        vec![2.0 * x[0] + 0.2, 1.5 * x[1] + 0.1, 1.0 * x[2] + 0.3]
    });

    let feasible_set = Simplex::new(3);
    let solver = FrankWolfeSolver::new(FrankWolfeConfig {
        max_iters: 50,
        tolerance: 1e-8,
    });

    let report = solver.solve(&operator, &feasible_set).unwrap();
    println!("solution = {:?}", report.solution);
    println!(
        "final_gap = {:?}",
        report.iterations.last().map(|it| it.gap.value)
    );
}

use crabscat::{
    FitOptions, Profile, Result, SingleParticleModel, Sphere, grid_search, write_fit_result,
};

fn main() -> Result<()> {
    let q: Vec<f64> = (1..=60).map(|index| index as f64 * 0.005).collect();

    let reference_model = SingleParticleModel::new(Sphere::new(35.0)?, 120.0, 0.02)?;
    let observed_intensity = reference_model
        .evaluate(&q)?
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            let offset = 1.0 + 0.01 * ((index % 5) as f64 - 2.0);
            value * offset
        })
        .collect();

    let observed = Profile::new(q.clone(), observed_intensity, Some(vec![0.05; q.len()]))?;

    // implement grid search
    let opts = FitOptions {
        start: 10.0,
        stop: 50.0,
        num_points: 200,
    };

    let best_fit = grid_search(&opts, &observed, |radius| {
        let model = SingleParticleModel::new(Sphere::new(radius)?, 120.0, 0.02)?;
        model.evaluate(observed.q())
    })?;

    // todo!();
    // let trial_model = SingleParticleModel::new(Sphere::new(32.0)?, 120.0, 0.02)?;
    // let predicted = trial_model.evaluate(&q)?;
    // let quality = evaluate_fit(&observed, &predicted, 3)?;

    println!("Best fit radius: {} A", best_fit.best_value());
    println!("chi^2 = {:.4}", best_fit.best_quality().chi_squared);
    println!(
        "reduced chi^2 = {:.4}",
        best_fit.best_quality().reduced_chi_squared
    );

    let _ = std::fs::create_dir_all("output")?;
    write_fit_result("output/grid_fit.dat", &observed, &best_fit)?;

    Ok(())
}

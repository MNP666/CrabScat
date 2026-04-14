use crabscat::form_factors::CoreShell;
use crabscat::{FitOptions, Profile, Result, SingleParticleModel, grid_search, write_fit_result};

fn main() -> Result<()> {
    let q: Vec<f64> = (1..=100).map(|index| index as f64 * 0.005).collect();

    let reference_formfactor = CoreShell::new(0.5, 2.0, 15.0, 5.0)?;
    let reference_model = SingleParticleModel::new(reference_formfactor, 100.0, 0.05)?;
    let observed_intensity = reference_model
        .evaluate(&q)? // returns I(q)
        .into_iter() // Takes ownership for collect, iterates of I(q)
        .enumerate() // provides index to the iteration like with python
        .map(|(index, value)| {
            let offset = 1.0 + 0.01 * ((index % 5) as f64 - 2.0);
            value * offset
        })
        .collect();

    let observed = Profile::new(q.clone(), observed_intensity, Some(vec![0.5; q.len()]))?;

    // implement grid search
    let opts = FitOptions::new(5.0, 50.0, 35)?;

    let best_fit = grid_search(&opts, &observed, |core_r| {
        let formf = CoreShell::new(0.5, 2.0, core_r, 5.0)?;
        let model = SingleParticleModel::new(formf, 100.0, 0.05)?;
        model.evaluate(observed.q())
    })?;

    println!("trial Core radius: {} A", best_fit.best_value());
    println!("chi^2 = {:.4}", best_fit.best_quality().chi_squared);
    println!(
        "reduced chi^2 = {:.4}",
        best_fit.best_quality().reduced_chi_squared
    );

    let _ = std::fs::create_dir_all("output")?;
    write_fit_result("output/core_grid_fit.dat", &observed, &best_fit)?;

    Ok(())
}

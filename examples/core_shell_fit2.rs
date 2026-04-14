use crabscat::{
    CoarseFineSearch, FitOptions, Profile, Result, SingleParticleModel, form_factors::CoreShell,
    write_fit_result,
};

fn main() -> Result<()> {
    let q: Vec<f64> = (0..100).map(|x| x as f64 / 1000.0 * 5.0).collect();
    let ff = CoreShell::new(1.0, 4.0, 38.0, 3.0)?;
    let model = SingleParticleModel::new(ff, 100.0, 0.5)?;
    let observed = model
        .evaluate(&q)? // returns I(q)
        .into_iter() // Takes ownership for collect, iterates of I(q)
        .enumerate() // provides index to the iteration like with python
        .map(|(index, value)| {
            let offset = 1.0 + 0.005 * ((index % 5) as f64 - 2.0);
            value * offset
        })
        .collect();
    let target = Profile::new(q.clone(), observed, Some(vec![0.5; q.len()]))?;

    let input = FitOptions::new(5.0, 50.0, 10)?;

    let search = CoarseFineSearch::new(input, 4, 2.0)?;

    let best_fit = search.fit(&target, |core_value| {
        let ff2 = CoreShell::new(1.0, 4.0, core_value, 3.0)?;
        let fit_profile = SingleParticleModel::new(ff2, 100.0, 0.5)?;
        fit_profile.evaluate(target.q())
    })?;

    println!("trial Core radius: {} A", best_fit.best_value());
    println!("chi^2 = {:.4}", best_fit.best_quality().chi_squared);
    println!(
        "reduced chi^2 = {:.4}",
        best_fit.best_quality().reduced_chi_squared
    );

    let _ = std::fs::create_dir_all("output")?;
    write_fit_result("output/core_grid_fit.dat", &target, &best_fit)?;

    Ok(())
}

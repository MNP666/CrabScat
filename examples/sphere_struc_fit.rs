use crabscat::{
    FitOptions, InteractingParticleModel, Lorentzian, Profile, Result, Sphere, grid_search,
    write_fit_result,
};

fn main() -> Result<()> {
    let q: Vec<f64> = (0..=100).map(|x| x as f64 * 0.005).collect();

    let radius = 32.0;
    let ff = Sphere::new(radius)?;
    // let xi = 3.0;
    let amplitude = -0.65;
    let sf = Lorentzian::new(60.0, amplitude)?;
    let target = InteractingParticleModel::new(100.0, 0.05, ff, sf)?;
    let obs_intensity: Vec<f64> = target
        .evaluate(&q)?
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            let offset = 1.0 + 0.01 * ((index % 5) as f64 - 2.0);
            value * offset
        })
        .collect();

    let obs_profile = Profile::new(q.clone(), obs_intensity, Some(vec![0.05; q.len()]))?;

    let opts = FitOptions::new(5.0, 125.0, 10)?;

    let best_fit = grid_search(&opts, &obs_profile, 3, |xi| {
        let fit_ff = Sphere::new(radius)?; // formfactor moved/comsumed
        let fit_fs = Lorentzian::new(xi, amplitude)?; // ampltide not consumed, a primitive with copy trait
        let fit_model = InteractingParticleModel::new(100.0, 0.05, fit_ff, fit_fs)?;
        fit_model.evaluate(obs_profile.q())
    })?;

    println!("Best fit correlation length: {} A", best_fit.best_value());
    println!("chi^2 = {:.4}", best_fit.best_quality().chi_squared);
    println!(
        "reduced chi^2 = {:.4}",
        best_fit.best_quality().reduced_chi_squared
    );

    let _ = std::fs::create_dir_all("output")?;
    write_fit_result("output/sphere_struc_fit.dat", &obs_profile, &best_fit)?;

    Ok(())
}

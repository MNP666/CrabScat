¬use crabscat::{FitOptions, Profile, Result, SingleParticleModel, Sphere, grid_search};

#[test]
fn grid_search_recovers_known_sphere_radius() -> Result<()> {
    let true_radius = 25.0;
    let scale = 120.0;
    let background = 0.02;
    let q: Vec<f64> = (1..=80).map(|index| index as f64 * 0.004).collect();

    let reference_model = SingleParticleModel::new(Sphere::new(true_radius)?, scale, background)?;
    let observed_intensity = reference_model.evaluate(&q)?;
    let point_count = q.len();
    let observed = Profile::new(q, observed_intensity, Some(vec![0.05; point_count]))?;

    let opts = FitOptions::new(20.0, 30.0, 11)?;
    let best_fit = grid_search(&opts, &observed, |radius| {
        let model = SingleParticleModel::new(Sphere::new(radius)?, scale, background)?;
        model.evaluate(observed.q())
    })?;

    assert!((best_fit.best_value() - true_radius).abs() < 1.0e-12);
    assert!(best_fit.best_quality().chi_squared < 1.0e-12);

    Ok(())
}

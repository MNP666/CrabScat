use crabscat::{Profile, Result, SingleParticleModel, Sphere, evaluate_fit, models::{Distribution, PolySphere}};

fn main() -> Result<()> {
    let q: Vec<f64> = (1..=60).map(|index| index as f64 * 0.005).collect();

    let dist = Distribution::Gaussian { mean: 30.0, std: 3.0, num_points: 12 };
    let reference_model = PolySphere::new(dist, 120.0, 0.02)?;
    let observed_intensity = reference_model.evaluate(&q)?;

    let observed = Profile::new(q.clone(), observed_intensity, Some(vec![0.05; q.len()]))?;

    let trial_model = SingleParticleModel::new(Sphere::new(30.0)?, 120.0, 0.02)?;
    let predicted = trial_model.evaluate(&q)?;
    let quality = evaluate_fit(&observed, &predicted, 3)?;

    println!("trial radius: {} A", trial_model.form_factor().radius());
    println!("chi^2 = {:.4}", quality.chi_squared);
    println!("reduced chi^2 = {:.4}", quality.reduced_chi_squared);

    Ok(())
}

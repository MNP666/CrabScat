use crabscat::form_factors::GaussChain;
use crabscat::{Profile, Result, SingleParticleModel, evaluate_fit};

fn main() -> Result<()> {
    let q: Vec<f64> = (1..=100).map(|index| index as f64 * 0.005).collect();

    let reference_formfactor = GaussChain::new(32.0)?;
    let reference_model = SingleParticleModel::new(reference_formfactor, 120.0, 0.5)?;
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

    let trial_formfactor = GaussChain::new(32.0)?;
    let trial_model = SingleParticleModel::new(trial_formfactor, 120.0, 0.5)?;
    let predicted = trial_model.evaluate(&q)?;
    let quality = evaluate_fit(&observed, &predicted, 3)?;

    println!("trial Rg: {} A", trial_model.form_factor().rg());
    println!("chi^2 = {:.4}", quality.chi_squared);
    println!("reduced chi^2 = {:.4}", quality.reduced_chi_squared);

    Ok(())
}

use crabscat::form_factors::FormFactor;
use crabscat::{Profile, Result, SingleParticleModel, Sphere, chi_squared, evaluate_fit, io};

#[test]
fn sphere_is_normalized_at_zero_q() -> Result<()> {
    let sphere = Sphere::new(25.0)?;
    let intensity = sphere.intensity_at(0.0)?;

    assert!((intensity - 1.0).abs() < 1.0e-12);
    Ok(())
}

#[test]
fn identical_model_has_zero_chi_squared() -> Result<()> {
    let q = vec![0.01, 0.02, 0.03, 0.04];
    let model = SingleParticleModel::new(Sphere::new(18.0)?, 10.0, 0.1)?;
    let predicted = model.evaluate(&q)?;
    let profile = Profile::new(q, predicted.clone(), Some(vec![0.1; predicted.len()]))?;

    let chi2 = chi_squared(&profile, &predicted)?;
    let quality = evaluate_fit(&profile, &predicted, 3)?;

    assert!(chi2.abs() < 1.0e-12);
    assert!(quality.reduced_chi_squared.abs() < 1.0e-12);
    Ok(())
}

#[test]
fn dat_parser_supports_three_columns() -> Result<()> {
    let profile = io::parse_dat(
        "
        # q I sigma
        0.01 12.0 0.5
        0.02 11.5 0.5
        0.03 10.7 0.5
        ",
    )?;

    assert_eq!(profile.len(), 3);
    assert_eq!(profile.q()[0], 0.01);
    assert_eq!(profile.intensity()[2], 10.7);
    assert_eq!(profile.sigma().expect("sigma should be present")[1], 0.5);

    Ok(())
}

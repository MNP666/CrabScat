use crabscat::{
    CoarseToFineSearch, CrabScatError, FitOptions, Profile, Result, SingleParticleModel, Sphere,
    grid_search,
};

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
    let best_fit = grid_search(&opts, &observed, 1, |radius| {
        let model = SingleParticleModel::new(Sphere::new(radius)?, scale, background)?;
        model.evaluate(observed.q())
    })?;

    assert!((best_fit.best_value() - true_radius).abs() < 1.0e-12);
    assert!(best_fit.best_quality().chi_squared < 1.0e-12);

    Ok(())
}

#[test]
fn coarse_to_fine_improves_over_initial_grid() -> Result<()> {
    let true_value = 26.0;
    let q: Vec<f64> = (0..8).map(|index| index as f64).collect();
    let observed = Profile::new(q, vec![true_value; 8], Some(vec![1.0; 8]))?;

    let initial = FitOptions::new(20.0, 30.0, 3)?;
    let coarse = grid_search(&initial, &observed, 1, |candidate| {
        Ok(vec![candidate; observed.len()])
    })?;

    let search = CoarseToFineSearch::new(initial, 3, 2.0)?;
    let refined = search.fit(&observed, 1, |candidate| {
        Ok(vec![candidate; observed.len()])
    })?;

    let coarse_error = (coarse.best_value() - true_value).abs();
    let refined_error = (refined.best_value() - true_value).abs();

    assert!(refined_error < coarse_error);
    assert!(refined.best_quality().chi_squared < coarse.best_quality().chi_squared);

    Ok(())
}

#[test]
fn coarse_to_fine_rejects_zero_levels() -> Result<()> {
    let initial = FitOptions::new(20.0, 30.0, 5)?;
    let error = match CoarseToFineSearch::new(initial, 0, 2.0) {
        Ok(_) => panic!("zero refinement levels should be rejected"),
        Err(error) => error,
    };

    match error {
        CrabScatError::InvalidParameter { name, .. } => assert_eq!(name, "levels"),
        other => panic!("expected InvalidParameter error, got {other:?}"),
    }

    Ok(())
}

#[test]
fn coarse_to_fine_rejects_invalid_shrink_factor() -> Result<()> {
    let initial = FitOptions::new(20.0, 30.0, 5)?;
    let error = match CoarseToFineSearch::new(initial, 3, 1.0) {
        Ok(_) => panic!("shrink_factor <= 1 should be rejected"),
        Err(error) => error,
    };

    match error {
        CrabScatError::InvalidParameter { name, .. } => assert_eq!(name, "shrink_factor"),
        other => panic!("expected InvalidParameter error, got {other:?}"),
    }

    Ok(())
}

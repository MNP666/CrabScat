use crate::data::Profile;
use crate::error::{CrabScatError, Result};

use super::FitQuality;

pub fn chi_squared(profile: &Profile, predicted: &[f64]) -> Result<f64> {
    if predicted.len() != profile.len() {
        return Err(CrabScatError::LengthMismatch {
            expected: profile.len(),
            found: predicted.len(),
            field: "predicted",
        });
    }

    let sum = match profile.sigma() {
        Some(sigma) => profile
            .intensity()
            .iter()
            .zip(predicted.iter())
            .zip(sigma.iter())
            .map(|((&observed, &model), &uncertainty)| {
                let residual = observed - model;
                residual.powi(2) / uncertainty.powi(2)
            })
            .sum(),
        None => profile
            .intensity()
            .iter()
            .zip(predicted.iter())
            .map(|(&observed, &model)| {
                let residual = observed - model;
                residual.powi(2)
            })
            .sum(),
    };

    Ok(sum)
}

pub fn evaluate_fit(
    profile: &Profile,
    predicted: &[f64],
    parameter_count: usize,
) -> Result<FitQuality> {
    let chi_squared = chi_squared(profile, predicted)?;

    if profile.len() <= parameter_count {
        return Err(CrabScatError::NotEnoughData {
            points: profile.len(),
            parameters: parameter_count,
        });
    }

    let degrees_of_freedom = profile.len() - parameter_count;
    let reduced_chi_squared = chi_squared / degrees_of_freedom as f64;

    Ok(FitQuality {
        chi_squared,
        reduced_chi_squared,
        degrees_of_freedom,
    })
}

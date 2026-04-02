use crate::error::{CrabScatError, Result};

#[derive(Clone, Debug)]
pub struct Profile {
    q: Vec<f64>,
    intensity: Vec<f64>,
    sigma: Option<Vec<f64>>,
}

impl Profile {
    pub fn new(q: Vec<f64>, intensity: Vec<f64>, sigma: Option<Vec<f64>>) -> Result<Self> {
        if q.is_empty() {
            return Err(CrabScatError::EmptyProfile);
        }

        if intensity.len() != q.len() {
            return Err(CrabScatError::LengthMismatch {
                expected: q.len(),
                found: intensity.len(),
                field: "intensity",
            });
        }

        if let Some(ref sigma_values) = sigma {
            if sigma_values.len() != q.len() {
                return Err(CrabScatError::LengthMismatch {
                    expected: q.len(),
                    found: sigma_values.len(),
                    field: "sigma",
                });
            }

            for (index, value) in sigma_values.iter().copied().enumerate() {
                if !value.is_finite() || value <= 0.0 {
                    return Err(CrabScatError::NonPositiveUncertainty { index, value });
                }
            }
        }

        Ok(Self {
            q,
            intensity,
            sigma,
        })
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn q(&self) -> &[f64] {
        &self.q
    }

    pub fn intensity(&self) -> &[f64] {
        &self.intensity
    }

    pub fn sigma(&self) -> Option<&[f64]> {
        self.sigma.as_deref()
    }
}

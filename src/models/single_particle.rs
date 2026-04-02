use crate::error::{CrabScatError, Result};
use crate::form_factors::FormFactor;

#[derive(Clone, Debug)]
pub struct SingleParticleModel<F> {
    form_factor: F,
    scale: f64,
    background: f64,
}

impl<F> SingleParticleModel<F>
where
    F: FormFactor,
{
    pub fn new(form_factor: F, scale: f64, background: f64) -> Result<Self> {
        if !scale.is_finite() || scale < 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "scale",
                value: scale,
                reason: "scale must be finite and non-negative",
            });
        }

        if !background.is_finite() {
            return Err(CrabScatError::InvalidParameter {
                name: "background",
                value: background,
                reason: "background must be finite",
            });
        }

        Ok(Self {
            form_factor,
            scale,
            background,
        })
    }

    pub fn form_factor(&self) -> &F {
        &self.form_factor
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn background(&self) -> f64 {
        self.background
    }

    pub fn intensity_at(&self, q: f64) -> f64 {
        self.scale * self.form_factor.intensity_at(q) + self.background
    }

    pub fn evaluate(&self, q: &[f64]) -> Vec<f64> {
        q.iter().map(|&value| self.intensity_at(value)).collect()
    }
}

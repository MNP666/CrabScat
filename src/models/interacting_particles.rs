use std::clone;

use crate::error::{CrabScatError, Result};
use crate::form_factors::FormFactor;
use crate::structure_factors::StructureFactor;

#[derive(Clone, Debug)]
pub struct InteractingParticleModel<F, S> {
    scale: f64,
    background: f64,
    form_factor: F,
    structure_factor: S,
}

impl<F, S> InteractingParticleModel<F, S>
where
    F: FormFactor,
    S: StructureFactor,
{
    pub fn new(scale: f64, background: f64, form_factor: F, structure_factor: S) -> Result<Self> {
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

        Ok(InteractingParticleModel {
            scale,
            background,
            form_factor,
            structure_factor,
        })
    }

    pub fn intensity_at(&self, q: f64) -> Result<f64> {
        Ok(self.scale
            * self.form_factor.intensity_at(q)?
            * self.structure_factor.structure_at(q)?
            + self.background)
    }

    pub fn evaluate(&self, q: &[f64]) -> Result<Vec<f64>> {
        q.iter().map(|&x| self.intensity_at(x)).collect()
    }
}

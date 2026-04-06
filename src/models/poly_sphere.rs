// use std::backtrace;

use crate::error::{CrabScatError, Result};
use crate::form_factors::{Sphere,FormFactor}; // FormFactor needed for trait implementaitons
use crate::models::{Distribution, WeightedPoint};


#[derive(Clone, Debug)]
pub struct PolySphere {
    distribution: Distribution,
    scale: f64,
    background: f64,
}

impl PolySphere {
    pub fn new(distribution: Distribution, scale: f64, background: f64) -> Result<Self> {
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
            distribution,
            scale,
            background,
        })
    }

    pub fn distribution(&self) -> &Distribution {
        &self.distribution
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn background(&self) -> f64 {
        self.background
    }

    pub fn intensity_at(&self, q: f64) -> Result<f64> {
        let poly_inputs = self.distribution.sample_points()?;
        let mut iq: f64 = 0.0;
        // OK to consume poly_inputs in iteration
        for WeightedPoint{value, weight } in poly_inputs {
            let sphere = Sphere::new(value)?;
            let i: f64 = sphere.intensity_at(q)?;
            iq += i * weight;

        };

        Ok(iq*self.scale+self.background)
    }

    pub fn evaluate(&self, q: &[f64]) -> Result<Vec<f64>> {
        q.iter().map(|&value| self.intensity_at(value)).collect()
    }
}
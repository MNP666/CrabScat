use crate::error::{CrabScatError, Result};

use super::FormFactor;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Result<Self> {
        if !radius.is_finite() || radius <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "radius",
                value: radius,
                reason: "radius must be positive and finite",
            });
        }

        Ok(Self { radius })
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn amplitude_at(&self, q: f64) -> f64 {
        let qr = q * self.radius;

        if qr.abs() < 1.0e-12 {
            return 1.0;
        }

        let numerator = qr.sin() - qr * qr.cos();
        3.0 * numerator / qr.powi(3)
    }
}

impl FormFactor for Sphere {
    fn intensity_at(&self, q: f64) -> f64 {
        let amplitude = self.amplitude_at(q);
        amplitude * amplitude
    }
}

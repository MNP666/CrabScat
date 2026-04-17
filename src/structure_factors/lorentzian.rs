use crate::error::{CrabScatError, Result};

use super::StructureFactor;

pub struct Lorentzian {
    xi: f64,
    amplitude: f64,
}

impl StructureFactor for Lorentzian {
    fn structure_at(&self, q: f64) -> Result<f64> {
        Ok(1.0 + self.amplitude / (1.0 + (self.xi * q).powi(2)))
    }
}

impl Lorentzian {
    pub fn new(xi: f64, amplitude: f64) -> Result<Self> {
        if !xi.is_finite() || xi <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "xi",
                value: xi,
                reason: "xi must be finite and > 0",
            });
        }

        if !amplitude.is_finite() || amplitude <= -1.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "ampltide",
                value: amplitude,
                reason: "amplitude must be > -1 and finite",
            });
        }

        Ok(Lorentzian { xi, amplitude })
    }

    pub fn xi(&self) -> f64 {
        self.xi
    }

    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }
}

use crate::error::{CrabScatError, Result};

use super::FormFactor;

#[derive(Clone, Copy, Debug)]
pub struct GaussChain {
    rg: f64,
}

impl GaussChain {
    pub fn new(rg: f64) -> Result<Self> {
        if !rg.is_finite() || rg <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "rg",
                value: rg,
                reason: "Radius of Gyration must be positive and finite",
            });
        }

        Ok(Self { rg })
    }

    pub fn rg(&self) -> f64 {
        self.rg
    }
}

impl FormFactor for GaussChain {
    fn intensity_at(&self, q: f64) -> Result<f64> {
        let u = q * q * self.rg * self.rg;

        if u == 0.0 {
            return Ok(1.0);
        }

        if u < 1.0e-3 {
            return Ok(1.0 - u / 3.0 + u * u / 12.0 - u * u * u / 60.0);
        }

        Ok(2.0 * ((-u).exp() - 1.0 + u) / (u * u))
    }
}

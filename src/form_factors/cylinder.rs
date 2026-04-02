use crate::error::{CrabScatError, Result};

#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    radius: f64,
    length: f64,
}

impl Cylinder {
    pub fn new(radius: f64, length: f64) -> Result<Self> {
        if !radius.is_finite() || radius <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "radius",
                value: radius,
                reason: "radius must be positive and finite",
            });
        }

        if !length.is_finite() || length <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "length",
                value: length,
                reason: "length must be positive and finite",
            });
        }

        Ok(Self { radius, length })
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn length(&self) -> f64 {
        self.length
    }
}

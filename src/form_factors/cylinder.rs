use crate::error::{CrabScatError, Result};
use crate::numerics::{DEFAULT_INTEGRATION_STEPS, simpson};
use libm::j1;

use super::FormFactor;

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

    #[allow(dead_code)]
    fn amplitude_at(&self, q: f64, mu: f64) -> f64 {
        let _axial = sinc((q * self.length * mu) / 2.0);
        let _radial = two_j1_over_x(q * self.radius * (1.0 - mu * mu).sqrt());

        // todo!("finish the cylinder amplitude implementation")
        _axial * _radial
    }
}

impl FormFactor for Cylinder {
    fn intensity_at(&self, q: f64) -> Result<f64> {
        let n = DEFAULT_INTEGRATION_STEPS;
        let p = simpson(
            |mu| {
                let a = self.amplitude_at(q, mu);
                a * a
            },
            0.0,
            1.0,
            n,
        )?;
        Ok(p)
    }
}

#[allow(dead_code)]
fn sinc(x: f64) -> f64 {
    if x.abs() < 1.0e-8 {
        return 1.0;
    }

    x.sin() / x
}

#[allow(dead_code)]
fn two_j1_over_x(x: f64) -> f64 {
    if x.abs() < 1.0e-4 {
        let x2 = x * x;
        return 1.0 - x2 / 8.0 + x2 * x2 / 192.0;
    }

    2.0 * j1(x) / x
}

#[cfg(test)]
mod tests {
    use super::two_j1_over_x;

    #[test]
    fn two_j1_over_x_has_the_correct_zero_limit() {
        assert!((two_j1_over_x(0.0) - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn two_j1_over_x_is_even() {
        let x = 0.7;
        assert!((two_j1_over_x(x) - two_j1_over_x(-x)).abs() < 1.0e-12);
    }
}

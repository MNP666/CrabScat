use crate::error::{CrabScatError, Result};
use std::f64::consts::PI;

use super::FormFactor;

#[derive(Clone, Copy, Debug)]
pub struct CoreShell {
    core_contrast: f64,
    shell_contrast: f64,
    core_radius: f64,
    shell_thickness: f64,
    normalization: f64
}

impl CoreShell {
    pub fn new(
        core_contrast: f64,
        shell_contrast: f64,
        core_radius: f64,
        shell_thickness: f64,
    ) -> Result<Self> {
        if !core_radius.is_finite() || core_radius <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "core_radius",
                value: core_radius,
                reason: "core_radius must be positive and finite",
            });
        }

        if !shell_thickness.is_finite() || shell_thickness < 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "shell_thickness",
                value: shell_thickness,
                reason: "shell_thickness must be zero/positive and finite",
            });
        }

        let fzero: f64 = (core_contrast - shell_contrast) * 4.0/3.0*PI*core_radius.powi(3)
                        + shell_contrast * 4.0/3.0*PI*(core_radius+shell_thickness).powi(3);
        
        if !fzero.is_finite() || fzero.abs() < 1e-12 {
            return Err(CrabScatError::InvalidParameter {
                name: "shell_contrast",
                value: shell_contrast,
                reason: "Forward amplitude most be non-zero",
            });
        }

        // No need to check shell_thickness + core are finite, because if both are each individually finite then so is their sum
        // f64 overflow is very unlikely for my usecases

        Ok(Self {
            core_contrast,
            shell_contrast,
            core_radius,
            shell_thickness,
            normalization: fzero
        })
    }

    pub fn outer_radius(&self) -> f64 {
        // Do I really need to return Result? ::new checks and validates the two values
        self.core_radius + self.shell_thickness
    }

    fn core_volume(&self) -> f64 {
        4.0 / 3.0 * PI * self.core_radius.powi(3)
    }

    fn outer_volume(&self) -> f64 {
        4.0 / 3.0 * PI * self.outer_radius().powi(3)
    }

    fn sph_kernel(qr: f64) -> f64 {
        if qr < 1.0e-12 {
            return 1.0
        }
        let numerator = qr.sin() - qr * qr.cos();
        3.0 * numerator / qr.powi(3)
    }

    pub fn amplitude_at(&self, q: f64) -> f64 {
        let qr_inner = q * self.core_radius;
        let qr_outer = q * self.outer_radius();



        let inner_amp = CoreShell::sph_kernel(qr_inner);
        let outer_amp = CoreShell::sph_kernel(qr_outer);

        let amp = (self.core_contrast - self.shell_contrast) * self.core_volume() * inner_amp
            + self.shell_contrast * self.outer_volume() * outer_amp;

        amp / self.normalization
    }
}

impl FormFactor for CoreShell {
    fn intensity_at(&self, q: f64) -> Result<f64> {
        let amp = self.amplitude_at(q);
        Ok(amp * amp)
    }
}

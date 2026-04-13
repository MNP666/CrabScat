mod core_shell;
mod cylinder;
mod gauss_chain;
mod sphere;

pub use core_shell::CoreShell;
pub use cylinder::Cylinder;
pub use gauss_chain::GaussChain;
pub use sphere::Sphere;

use crate::error::Result;

pub trait FormFactor {
    fn intensity_at(&self, q: f64) -> Result<f64>;

    fn evaluate(&self, q: &[f64]) -> Result<Vec<f64>> {
        q.iter().map(|&value| self.intensity_at(value)).collect()
    }
}

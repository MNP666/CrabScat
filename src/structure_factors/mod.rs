mod lorentzian;

pub use lorentzian::Lorentzian;

use crate::error::Result;

pub trait StructureFactor {
    fn structure_at(&self, q: f64) -> Result<f64>;

    fn evaluate(&self, q: &[f64]) -> Result<Vec<f64>> {
        q.iter().map(|&x| self.structure_at(x)).collect()
    }
}

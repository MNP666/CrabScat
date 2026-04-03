mod cylinder;
mod gauss_chain;
mod sphere;

pub use cylinder::Cylinder;
pub use gauss_chain::GaussChain;
pub use sphere::Sphere;

pub trait FormFactor {
    fn intensity_at(&self, q: f64) -> f64;

    fn evaluate(&self, q: &[f64]) -> Vec<f64> {
        q.iter().map(|&value| self.intensity_at(value)).collect()
    }
}

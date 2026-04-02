pub mod data;
pub mod error;
pub mod fitting;
pub mod form_factors;
pub mod io;
pub mod models;

pub use data::Profile;
pub use error::{CrabScatError, Result};
pub use fitting::{FitQuality, chi_squared, evaluate_fit};
pub use form_factors::{Cylinder, Sphere};
pub use models::SingleParticleModel;

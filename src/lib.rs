pub mod data;
pub mod error;
pub mod fitting;
pub mod form_factors;
pub mod io;
pub mod models;
pub mod numerics;

pub use data::Profile;
pub use error::{CrabScatError, Result};
pub use fitting::{FitOptions, FitQuality, chi_squared, evaluate_fit, grid_search};
pub use form_factors::{Cylinder, Sphere};
pub use io::write_fit_result;
pub use models::{PolySphere, SingleParticleModel};
pub use numerics::{DEFAULT_INTEGRATION_STEPS, integrate_1d, simpson};

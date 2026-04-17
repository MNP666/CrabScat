pub mod data;
pub mod error;
pub mod fitting;
pub mod form_factors;
pub mod io;
pub mod models;
pub mod numerics;
pub mod structure_factors;

pub use data::Profile;
pub use error::{CrabScatError, Result};
pub use fitting::{
    CoarseToFineSearch, FitOptions, FitQuality, chi_squared, evaluate_fit, grid_search,
};
pub use form_factors::{Cylinder, Sphere};
pub use structure_factors::Lorentzian;
pub use io::write_fit_result;
pub use models::{PolySphere, SingleParticleModel, InteractingParticleModel};
pub use numerics::{DEFAULT_INTEGRATION_STEPS, integrate_1d, simpson};

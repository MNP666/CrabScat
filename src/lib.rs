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
pub use io::{write_fit_result, write_gp_result};
pub use models::{InteractingParticleModel, PolySphere, SingleParticleModel};
pub use numerics::{DEFAULT_INTEGRATION_STEPS, covar, integrate_1d, simpson};
pub use structure_factors::Lorentzian;

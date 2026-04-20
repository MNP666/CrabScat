mod gaussian_process;
mod integration;

pub use gaussian_process::covar;
pub use integration::{DEFAULT_INTEGRATION_STEPS, integrate_1d, simpson};

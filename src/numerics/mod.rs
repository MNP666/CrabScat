mod gaussian_process;
mod integration;

pub use gaussian_process::{rbf_covariance, RbfGaussianProcess,RbfGPResult} ;
pub use integration::{DEFAULT_INTEGRATION_STEPS, integrate_1d, simpson};

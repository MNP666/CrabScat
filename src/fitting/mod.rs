mod grid_search;
mod least_squares;
mod result;

pub use grid_search::{CoarseFineSearch, FitOptions, GridSearchResult, grid_search};
pub use least_squares::{chi_squared, evaluate_fit};
pub use result::FitQuality;

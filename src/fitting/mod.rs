mod coarse_to_fine;
mod grid_search;
mod least_squares;
mod result;

pub use coarse_to_fine::CoarseToFineSearch;
pub use grid_search::{FitOptions, GridSearchResult, grid_search};
pub use least_squares::{chi_squared, evaluate_fit};
pub use result::FitQuality;

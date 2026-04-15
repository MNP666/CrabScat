use crate::error::Result;
use crate::{CrabScatError, Profile};

use super::{FitOptions, GridSearchResult, grid_search};

pub struct CoarseToFineSearch {
    initial: FitOptions,
    levels: usize,
    shrink_factor: f64,
}

impl CoarseToFineSearch {
    pub fn new(initial: FitOptions, levels: usize, shrink_factor: f64) -> Result<Self> {
        if levels == 0 {
            return Err(CrabScatError::InvalidParameter {
                name: "levels",
                value: levels as f64,
                reason: "levels must be 1 or larger",
            });
        }

        if !shrink_factor.is_finite() || shrink_factor <= 1.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "shrink_factor",
                value: shrink_factor,
                reason: "shrink_factor must be finite and greater than 1.0",
            });
        }

        Ok(Self {
            initial,
            levels,
            shrink_factor,
        })
    }

    pub fn fit<F>(
        &self,
        data: &Profile,
        parameter_count: usize,
        model_for_value: F,
    ) -> Result<GridSearchResult>
    where
        F: Fn(f64) -> Result<Vec<f64>>,
    {
        let mut current_opts = self.initial.clone();
        let num_points = current_opts.num_points();
        let mut result: Option<GridSearchResult> = None;

        for _ in 0..self.levels {
            let fit = grid_search(&current_opts, data, parameter_count, &model_for_value)?;

            let center = fit.best_value();
            let new_width = (current_opts.stop() - current_opts.start()) / self.shrink_factor;
            current_opts = FitOptions::new(
                center - new_width / 2.0,
                center + new_width / 2.0,
                num_points,
            )?;

            result = Some(fit);
        }

        result.ok_or(CrabScatError::NoOptimum {
            reason: "coarse-to-fine search did not run any levels",
        })
    }
}

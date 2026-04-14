// use std::num;

use crate::CrabScatError;
use crate::error::Result;
use crate::{FitQuality, Profile, evaluate_fit};

pub struct GridSearchResult {
    best_value: f64,
    best_quality: FitQuality,
    best_predicted: Vec<f64>,
}



impl GridSearchResult {
    pub fn new(best_value: f64, best_quality: FitQuality, best_predicted: Vec<f64>) -> Self {
        Self {
            best_value,
            best_quality,
            best_predicted,
        }
    }

    pub fn best_value(&self) -> f64 {
        self.best_value
    }

    pub fn best_quality(&self) -> FitQuality {
        self.best_quality
    }

    pub fn best_predicted(&self) -> &[f64] {
        &self.best_predicted
    }
}
#[derive(Clone, Debug)]
pub struct FitOptions {
    start: f64,
    stop: f64,
    num_points: usize,
}

impl Default for FitOptions {
    fn default() -> Self {
        FitOptions {
            start: 5.0,
            stop: 60.0,
            num_points: 200,
        }
    }
}
impl FitOptions {
    pub fn new(start: f64, stop: f64, num_points: usize) -> Result<FitOptions> {
        if stop <= start || !stop.is_finite() || !start.is_finite() {
            return Err(CrabScatError::InvalidParameter {
                name: "Start/Stop",
                value: stop,
                reason: "Start and stop must be finit and Stop value must be larger than start",
            });
        };

        if num_points < 2 {
            return Err(CrabScatError::InvalidParameter {
                name: "num_points",
                value: num_points as f64,
                reason: "At least two points are needed for a scan",
            });
        };

        Ok(FitOptions {
            start,
            stop,
            num_points,
        })
    }

    pub fn start(&self) -> f64 {
        self.start
    }

    pub fn stop(&self) -> f64 {
        self.stop
    }

    pub fn num_points(&self) -> usize {
        self.num_points
    }
}

pub fn grid_search<F>(
    opts: &FitOptions,
    data: &Profile,
    model_for_value: F,
) -> Result<GridSearchResult>
where
    F: Fn(f64) -> Result<Vec<f64>>,
{
    let delta = (opts.stop - opts.start) / (opts.num_points as f64 - 1.0);
    let mut best_result: Option<GridSearchResult> = None;

    for index in 0..opts.num_points {
        let val = opts.start + index as f64 * delta;
        let predicted = model_for_value(val)?;
        let tmp_result = evaluate_fit(&data, &predicted, 3)?;

        let is_better: bool = match &best_result {
            None => true,
            Some(result) => tmp_result.chi_squared < result.best_quality.chi_squared,
        };

        if is_better {
            best_result = Some(GridSearchResult::new(val, tmp_result, predicted));
        }
    }

    // assert that best_chi2 is different with 1e-2 tolerance from initial values
    best_result.ok_or(CrabScatError::NoOptimum {
        reason: "No optimum found",
    })
}


pub struct CoarseFineSearch {
    initial: FitOptions,
    levels: usize,
    shrink_factor: f64
}

impl CoarseFineSearch {
    pub fn new(initial: FitOptions, levels: usize, shrink_factor: f64) -> Result<Self> {
        // does not consume FitOptions
        if levels == 0 {
            return Err(CrabScatError::InvalidParameter { 
                name: "levels", 
                value: levels as f64, 
                reason: "Levels must be integer 1 or above" });
        };

        if !shrink_factor.is_finite() || shrink_factor <= 1.0 {
            return Err(CrabScatError::InvalidParameter { 
                name: "shrink_factor", 
                value: shrink_factor, 
                reason: "shrink factor must be larger than 1.0" });
        };

        Ok(CoarseFineSearch{initial, levels, shrink_factor})
    }

    pub fn fit<F>(&self, data: &Profile, model_for_value: F) -> Result<GridSearchResult> 
    where 
    F: Fn(f64) -> Result<Vec<f64>> {

        let mut current_opts = self.initial.clone();
        let num_points = current_opts.num_points();
        let mut result: Option<GridSearchResult> = None;

        for _ in 0..self.levels {
            let fit = grid_search(&current_opts, data, &model_for_value)?;


            let center = fit.best_value();
            let new_width = (current_opts.stop()-current_opts.start())/self.shrink_factor;
            current_opts = FitOptions::new(
                center-new_width/2.0,
                center+new_width/2.0,
                num_points,
            )?;

            result = Some(fit)

        }


        result.ok_or(CrabScatError::NoOptimum { reason: "coarse-to-fine search did not run any levels" })


    }

}
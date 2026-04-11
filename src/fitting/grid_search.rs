use crate::CrabScatError;
use crate::error::Result;
use crate::{FitQuality, Profile, evaluate_fit};

pub struct GridSearchResult {
    pub best_value: f64,
    pub best_quality: FitQuality,
    pub best_predicted: Vec<f64>,
}

impl GridSearchResult {
    pub fn best_predicted(&self) -> &[f64] {
        &self.best_predicted
    }
}

pub struct FitOptions {
    pub start: f64,
    pub stop: f64,
    pub num_points: usize,
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
            best_result = Some(GridSearchResult {
                best_value: val,
                best_quality: tmp_result,
                best_predicted: predicted,
            });
        }
    }

    // assert that best_chi2 is different with 1e-2 tolerance from initial values
    best_result.ok_or(CrabScatError::NoOptimum {
        reason: "No optimum found",
    })
}

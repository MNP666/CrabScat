use crate::error::{CrabScatError, Result};
use ndarray::Array2;

fn rbf(x1: f64, x2: f64, length: f64, amp: f64) -> f64 {
    amp.powi(2) * (-0.5 * ((x1 - x2) / length).powi(2)).exp()
}

pub fn rbf_covariance(x_left: &[f64], x_right: &[f64], length: f64, amp: f64) -> Result<Array2<f64>> {
    // do not enforce same size. Correct for training, but not necessarily correct for fitting/prediction.
    // if x_left.len() != x_right.len() {
    //     return Err(CrabScatError::LengthMismatch { expected: x_left.len(), found: x_right.len(), field: "Ensure x_left and x_right are same size" });
    // }

    if !length.is_finite() || length <= 0.0 {
        return Err(CrabScatError::InvalidParameter {
            name: "length",
            value: length,
            reason: "The length scale must be finite and stricly positive",
        });
    }

    if !amp.is_finite() || amp <= 0.0 {
        return Err(CrabScatError::InvalidParameter {
            name: "amp",
            value: length,
            reason: "The ampltidue must be finite stricly positive",
        });
    }

    let mut k_matrix: Array2<f64> = Array2::zeros((x_left.len(), x_right.len()));
    for (row_index, xleft) in x_left.iter().enumerate() {
        for (column_index, xright) in x_right.iter().enumerate() {
            k_matrix[[row_index, column_index]] = rbf(*xleft, *xright, length, amp);
        }
    }

    Ok(k_matrix)
}

#[derive(Clone, Debug)]
pub struct RbfGaussianProcess {
    amplitude: f64,
    length_scale: f64,
    noise_sigma: f64,
}

pub struct RbfGPResult {
    
}
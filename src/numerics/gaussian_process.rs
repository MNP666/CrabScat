use crate::error::{CrabScatError, Result};
use linfa_linalg::{cholesky::Cholesky, triangular::SolveTriangular, triangular::UPLO};
use ndarray::Array2;

fn rbf(x1: f64, x2: f64, length: f64, amp: f64) -> f64 {
    amp.powi(2) * (-0.5 * ((x1 - x2) / length).powi(2)).exp()
}

pub fn rbf_covariance(
    x_left: &[f64],
    x_right: &[f64],
    length: f64,
    amp: f64,
) -> Result<Array2<f64>> {
    // do not enforce same size. Correct for training, but not necessarily correct for fitting/prediction.
    // if x_left.len() != x_right.len() {
    //     return Err(CrabScatError::LengthMismatch { expected: x_left.len(), found: x_right.len(), field: "Ensure x_left and x_right are same size" });
    // }

    // commenting out validation, do it in the .fit method
    // if !length.is_finite() || length <= 0.0 {
    //     return Err(CrabScatError::InvalidParameter {
    //         name: "length",
    //         value: length,
    //         reason: "The length scale must be finite and stricly positive",
    //     });
    // }

    // if !amp.is_finite() || amp <= 0.0 {
    //     return Err(CrabScatError::InvalidParameter {
    //         name: "amp",
    //         value: length,
    //         reason: "The ampltidue must be finite stricly positive",
    //     });
    // }

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
    l: Array2<f64>,
    alpha: Array2<f64>,
    x: Vec<f64>,
}

impl RbfGaussianProcess {
    pub fn fit(
        x_train: &[f64],
        y_train: &[f64],
        amplitude: f64,
        length_scale: f64,
        noise_sigma: f64,
    ) -> Result<Self> {
        if !length_scale.is_finite() || length_scale <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "length",
                value: length_scale,
                reason: "The length scale must be finite and stricly positive",
            });
        }

        if !amplitude.is_finite() || amplitude <= 0.0 {
            return Err(CrabScatError::InvalidParameter {
                name: "amp",
                value: amplitude,
                reason: "The ampltidue must be finite stricly positive",
            });
        }
        if x_train.len() != y_train.len() {
            return Err(CrabScatError::LengthMismatch {
                expected: x_train.len(),
                found: y_train.len(),
                field: "x and y train must have same length",
            });
        }

        let mut k_train: Array2<f64> = rbf_covariance(x_train, x_train, length_scale, amplitude)?;
        for index in 0..x_train.len() {
            k_train[[index, index]] += noise_sigma.powi(2) + 1e-10 // +add the variance to the diagonal + some jitter 
        }

        let l = k_train.cholesky()?;
        let y_col = Array2::from_shape_vec((y_train.len(), 1), y_train.to_vec()).unwrap(); // this should never give an error
        let z = l.solve_triangular(&y_col, UPLO::Lower)?;
        let alpha = l.t().solve_triangular(&z, UPLO::Upper)?;
        Ok(RbfGaussianProcess {
            amplitude,
            length_scale,
            noise_sigma,
            l,
            alpha,
            x: x_train.to_vec(),
        })
    }

    pub fn predict(&self, x_pred: &[f64]) -> Result<RbfGPResult> {
        let k_star = rbf_covariance(&x_pred, &self.x, self.length_scale, self.amplitude)?;
        // new y_predict mean
        let mean = k_star.dot(&self.alpha);
        let mean = mean.column(0).to_vec();
        // confidence estimate
        let v_mat = self.l.solve_triangular(&k_star.t(), UPLO::Lower)?;
        let mut stdev = vec![f64::NAN; x_pred.len()];
        for (index_j, _j) in x_pred.iter().enumerate() {
            let reduction: f64 = (0..v_mat.nrows())
                .map(|index_i| v_mat[[index_i, index_j]].powi(2))
                .sum();
            let variance = (self.amplitude.powi(2) - reduction).max(0.0);
            stdev[index_j] = variance.sqrt();
        }
        let mut upper = vec![f64::NAN; stdev.len()];
        let mut lower = vec![f64::NAN; stdev.len()];

        for (index, (std_val, mean_val)) in stdev.iter().zip(mean.iter()).enumerate() {
            upper[index] = mean_val + std_val * 1.96;
            lower[index] = mean_val - std_val * 1.96;
        }

        Ok(RbfGPResult {
            x: x_pred.to_vec(),
            mean,
            latent_std: stdev,
            upper,
            lower,
        })
    }

    pub fn length_scale(&self) -> f64 {
        self.length_scale
    }

    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn noise_sigma(&self) -> f64 {
        self.noise_sigma
    }
}

pub struct RbfGPResult {
    pub x: Vec<f64>,
    pub mean: Vec<f64>,
    pub latent_std: Vec<f64>,
    pub upper: Vec<f64>,
    pub lower: Vec<f64>,
}

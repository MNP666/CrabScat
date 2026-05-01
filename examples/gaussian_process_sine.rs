use core::f64;

use crabscat::{RbfGaussianProcess, Result, write_gp_result};
// use linfa_linalg::{cholesky::Cholesky, triangular::SolveTriangular, triangular::UPLO};
// use ndarray::Array2;
use rand_distr::{Distribution, Normal};
// use ;

fn main() -> Result<()> {
    // Generate noise data
    let num_points: usize = 80;
    let sigma = 0.1;
    let x: Vec<f64> = (0..num_points)
        .map(|value| value as f64 / num_points as f64 * 10.0)
        .collect();
    let y: Vec<f64> = x.iter().map(|value| value.sin()).collect();
    let normal = Normal::new(0., 1.).unwrap();
    let y_noise: Vec<f64> = y
        .iter()
        .map(|yval| yval + sigma * normal.sample(&mut rand::rng()))
        .collect();

    // build covariance k_matrix
    let length = 1.0;
    let amplitude = 1.0;

    let rbf_process = RbfGaussianProcess::fit(&x, &y_noise, amplitude,
    length, sigma)?;


    // prediction
    // let x_pred = x.clone();
    let x_pred_num = 40;
    let x_pred: Vec<f64> = (0..x_pred_num)
        .map(|value| value as f64 / x_pred_num as f64 * 10.0)
        .collect();

    let gp_result = rbf_process.predict(&x_pred)?;

    write_gp_result(
        "output/gp_sine_test.dat",
        &x,
        &y_noise,
        &rbf_process,
        &gp_result,

    )?;

    Ok(())
}

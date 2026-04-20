use core::f64;

use crabscat::{Result, rbf_covariance, write_gp_result};
use linfa_linalg::{cholesky::Cholesky, triangular::SolveTriangular, triangular::UPLO};
use ndarray::Array2;
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
    let mut k_train: Array2<f64> = rbf_covariance(&x, &x, length, amplitude)?;
    for index in 0..num_points {
        k_train[[index, index]] += sigma.powi(2) + 1e-10 // +add the variance to the diagonal + some jitter 
    }

    // Decomposition
    let l = k_train.cholesky()?;

    let y_col = Array2::from_shape_vec((num_points, 1), y_noise).unwrap(); // this should never give an error

    let z = l
        .solve_triangular(&y_col, UPLO::Lower)?;

    let alpha = l
        .t()
        .solve_triangular(&z, UPLO::Upper)?;

    // prediction
    // let x_pred = x.clone();
    let x_pred_num = 40;
    let x_pred: Vec<f64> = (0..x_pred_num)
        .map(|value| value as f64 / x_pred_num as f64 * 10.0)
        .collect();
    let k_star = rbf_covariance(&x_pred, &x, length, amplitude)?;
    let mean = k_star.dot(&alpha);

    // Confidence
    let v_mat = l
        .solve_triangular(&k_star.t(), UPLO::Lower)?;

    let mut stdev = vec![f64::NAN; x_pred.len()];
    for (index_j, _j) in x_pred.iter().enumerate() {
        let reduction: f64 = (0..v_mat.nrows())
        .map(|index_i| v_mat[[index_i, index_j]].powi(2))
        .sum();
        let variance = (amplitude.powi(2) - reduction).max(0.0);
        stdev[index_j] = variance.sqrt();
    }

    write_gp_result("output/gp_sine_test.dat", &x, &x_pred, &y_col, &mean, &stdev)?;

    Ok(())
}

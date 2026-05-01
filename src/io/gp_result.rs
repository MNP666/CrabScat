use crate::{RbfGPResult, RbfGaussianProcess};
use crate::error::Result;
// use ndarray::Array2;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write_gp_result(
    path: impl AsRef<Path>,
    x_train: &[f64],
    y_train: &[f64],
    process: &RbfGaussianProcess,
    prediction: &RbfGPResult,
) -> Result<()> {
    // let header = "x_train\ty_train\tx_pred\ty_pred\tupper\tlower";
    let header = "Kind\tX\tY\tSTD\tUpper\tLower\tLength_scale\tAmplitude\tNoise";

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{header}")?;
     // let mut x_pred_val = f64::NAN;´
    // let mut y_pred_val = f64::NAN;
    // let mut upper_val = f64::NAN;
    // let mut lower_val = f64::NAN;

    for i in 0..x_train.len() {
        // if i < prediction.x.len() {
        //     x_pred_val = prediction.x[i];
        //     y_pred_val = prediction.mean[i];
        //     upper_val = prediction.upper[i];
        //     lower_val = prediction.lower[i];
        // }

        writeln!(
            writer,
            "{}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8}\t{:.8e}\t{:.8}",
            "Training",
            x_train[i],
            y_train[i],
            f64::NAN,f64::NAN,f64::NAN,f64::NAN,f64::NAN,f64::NAN
 
        )?;
    }

    for i in 0..prediction.x.len() {
        writeln!(
            writer,
            "{}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8}\t{:.8e}\t{:.8}",
            "Prediction",
            prediction.x[i],
            prediction.mean[i],
            prediction.latent_std[i],
            prediction.upper[i],
            prediction.lower[i],
            process.length_scale(),
            process.amplitude(),
            process.noise_sigma(),
 
        )?;
    }

    Ok(())
}

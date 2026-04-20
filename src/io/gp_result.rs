use crate::error::Result;
use ndarray::Array2;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write_gp_result(
    path: impl AsRef<Path>,
    x_train: &[f64],
    x_pred: &[f64],
    data: &Array2<f64>,
    fit: &Array2<f64>,
    stdev: &[f64],
) -> Result<()> {
    let header = "x_train\ty_train\tx_pred\ty_pred\tsigma";

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{header}")?;

    let mut x_pred_val = f64::NAN;
    let mut y_pred_val = f64::NAN;
    let mut stdev_pred_val = f64::NAN;

    for i in 0..x_train.len() {
        if i < x_pred.len() {
            x_pred_val = x_pred[i];
            y_pred_val = fit[[i, 0]];
            stdev_pred_val = stdev[i];
        }

        writeln!(
            writer,
            "{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}",
            x_train[i],
            data[[i, 0]],
            x_pred_val,
            y_pred_val,
            stdev_pred_val,
        )?;
    }

    Ok(())
}

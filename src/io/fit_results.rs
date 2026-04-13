use crate::data::Profile;
use crate::error::{CrabScatError, Result};
use crate::fitting::GridSearchResult;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write_fit_result(
    path: impl AsRef<Path>,
    data: &Profile,
    fit: &GridSearchResult,
) -> Result<()> {
    let header = "q\tIq_obs\tIq_fit\tsigma\tresiduals";

    let q = data.q();
    let observed = data.intensity();
    let predicted = fit.best_predicted();
    let sigma = data.sigma();

    if predicted.len() != data.len() {
        return Err(CrabScatError::LengthMismatch {
            expected: data.len(),
            found: predicted.len(),
            field: "best_predicted",
        });
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{header}")?;

    for i in 0..data.len() {
        let sigma_value = match sigma {
            None => f64::NAN,
            Some(values) => values[i],
        };

        writeln!(
            writer,
            "{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}\t{:.8e}",
            q[i],
            observed[i],
            predicted[i],
            sigma_value,
            observed[i] - predicted[i]
        )?;
    }

    Ok(())
}

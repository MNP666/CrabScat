use crabscat::{
    CrabScatError, FitQuality, Profile, Result, fitting::GridSearchResult, write_fit_result,
};

fn fit(predicted: Vec<f64>) -> GridSearchResult {
    GridSearchResult::new(
        1.0,
        FitQuality {
            chi_squared: 0.0,
            reduced_chi_squared: 0.0,
            degrees_of_freedom: 1,
        },
        predicted,
    )
}

#[test]
fn write_fit_result_writes_header_rows_and_sigma() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("fit.dat");

    let profile = Profile::new(vec![0.01, 0.02], vec![10.0, 8.0], Some(vec![0.5, 0.25]))?;

    write_fit_result(&path, &profile, &fit(vec![9.5, 8.25]))?;

    let written = std::fs::read_to_string(path)?;
    assert_eq!(
        written,
        "q\tIq_obs\tIq_fit\tsigma\tresiduals\n\
         1.00000000e-2\t1.00000000e1\t9.50000000e0\t5.00000000e-1\t5.00000000e-1\n\
         2.00000000e-2\t8.00000000e0\t8.25000000e0\t2.50000000e-1\t-2.50000000e-1\n"
    );

    Ok(())
}

#[test]
fn write_fit_result_writes_header_rows_no_sigma() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("fit.dat");

    let profile = Profile::new(vec![0.01, 0.02], vec![10.0, 8.0], None)?;

    write_fit_result(&path, &profile, &fit(vec![9.5, 8.25]))?;

    let written = std::fs::read_to_string(path)?;
    assert_eq!(
        written,
        "q\tIq_obs\tIq_fit\tsigma\tresiduals\n\
         1.00000000e-2\t1.00000000e1\t9.50000000e0\tNaN\t5.00000000e-1\n\
         2.00000000e-2\t8.00000000e0\t8.25000000e0\tNaN\t-2.50000000e-1\n"
    );

    Ok(())
}

#[test]
fn write_fit_result_rejects_mismatched_predicted_length() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("fit.dat");

    let profile = Profile::new(vec![0.01, 0.02], vec![10.0, 8.0], Some(vec![0.5, 0.25]))?;

    let fit = GridSearchResult::new(
        1.0,
        FitQuality {
            chi_squared: 0.0,
            reduced_chi_squared: 0.0,
            degrees_of_freedom: 1,
        },
        vec![9.5], // wrong length: profile has 2 points
    );

    let err = write_fit_result(&path, &profile, &fit)
        .expect_err("mismatched predicted length should be rejected");

    match err {
        CrabScatError::LengthMismatch {
            expected,
            found,
            field,
        } => {
            assert_eq!(expected, 2);
            assert_eq!(found, 1);
            assert_eq!(field, "best_predicted");
        }
        other => panic!("expected LengthMismatch error, got {other:?}"),
    }

    assert!(
        !path.exists(),
        "invalid fit data should be rejected before creating the output file"
    );

    Ok(())
}

#[test]
fn write_fit_result_returns_io_error_when_file_cannot_be_created() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("missing-directory").join("fit.dat");

    let profile = Profile::new(vec![0.01, 0.02], vec![10.0, 8.0], Some(vec![0.5, 0.25]))?;

    let fit = GridSearchResult::new(
        1.0,
        FitQuality {
            chi_squared: 0.0,
            reduced_chi_squared: 0.0,
            degrees_of_freedom: 1,
        },
        vec![9.5, 8.25],
    );

    let err = write_fit_result(&path, &profile, &fit)
        .expect_err("writing inside a missing directory should fail");

    match err {
        CrabScatError::Io(error) => {
            assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        }
        other => panic!("expected Io error, got {other:?}"),
    }

    Ok(())
}

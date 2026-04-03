use crate::error::{CrabScatError, Result};

pub const DEFAULT_INTEGRATION_STEPS: usize = 256;

pub fn integrate_1d<F>(f: F, a: f64, b: f64, n: usize) -> Result<f64>
where
    F: Fn(f64) -> f64,
{
    simpson(f, a, b, n)
}

pub fn simpson<F>(f: F, a: f64, b: f64, n: usize) -> Result<f64>
where
    F: Fn(f64) -> f64,
{
    validate_interval(a, b)?;
    validate_simpson_steps(n)?;

    let h = (b - a) / n as f64;
    let fa = evaluate_integrand(&f, a)?;
    let fb = evaluate_integrand(&f, b)?;

    // `fa`, `fb`, and `h` are computed up front so the later implementation can
    // focus on the Simpson accumulation logic.
    let _ = (fa, fb, h);

    todo!("implement composite Simpson integration")
}

fn validate_interval(a: f64, b: f64) -> Result<()> {
    if !a.is_finite() || !b.is_finite() {
        return Err(CrabScatError::InvalidIntegrationInterval {
            a,
            b,
            reason: "bounds must be finite",
        });
    }

    if b <= a {
        return Err(CrabScatError::InvalidIntegrationInterval {
            a,
            b,
            reason: "upper bound must be greater than lower bound",
        });
    }

    Ok(())
}

fn validate_simpson_steps(n: usize) -> Result<()> {
    if n == 0 {
        return Err(CrabScatError::InvalidIntegrationSteps {
            n,
            reason: "step count must be greater than zero",
        });
    }

    if n % 2 != 0 {
        return Err(CrabScatError::InvalidIntegrationSteps {
            n,
            reason: "Simpson's rule requires an even number of steps",
        });
    }

    Ok(())
}

fn evaluate_integrand<F>(f: &F, x: f64) -> Result<f64>
where
    F: Fn(f64) -> f64,
{
    let value = f(x);

    if !value.is_finite() {
        return Err(CrabScatError::NonFiniteIntegrand { x, value });
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{integrate_1d, simpson};
    use crate::error::CrabScatError;

    #[test]
    fn integrate_1d_rejects_odd_step_count() {
        let error = integrate_1d(|x| x, 0.0, 1.0, 5).expect_err("odd steps should fail");

        match error {
            CrabScatError::InvalidIntegrationSteps { n, .. } => assert_eq!(n, 5),
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn simpson_rejects_invalid_interval() {
        let error = simpson(|x| x, 1.0, 0.0, 10).expect_err("reversed interval should fail");

        match error {
            CrabScatError::InvalidIntegrationInterval { a, b, .. } => {
                assert_eq!(a, 1.0);
                assert_eq!(b, 0.0);
            }
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn simpson_rejects_non_finite_endpoint() {
        let error =
            simpson(|_| f64::INFINITY, 0.0, 1.0, 10).expect_err("non-finite endpoint should fail");

        match error {
            CrabScatError::NonFiniteIntegrand { .. } => {}
            other => panic!("unexpected error: {other}"),
        }
    }
}

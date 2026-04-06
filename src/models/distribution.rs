use crate::error::{CrabScatError, Result};

pub struct WeightedPoint {
    pub value: f64,
    pub weight: f64,
}

const GAUSSIAN_CUTOFF: f64 = 2.0;

#[derive(Clone, Debug)]
pub enum Distribution {
    Gaussian {
        mean: f64,
        std: f64,
        num_points: usize,
    },
    Uniform {
        start: f64,
        stop: f64,
        num_points: usize,
    },
}

impl Distribution {
    pub fn sample_points(&self) -> Result<Vec<WeightedPoint>> {
        match self {
            Self::Gaussian {
                mean,
                std,
                num_points,
            } => {
                let mean = *mean;
                let std = *std;
                let num_points = *num_points;
                validate_gaussian(mean, std, num_points)?;
                let np_float = num_points as f64;

                let start = mean - GAUSSIAN_CUTOFF * std;
                let stop = mean + GAUSSIAN_CUTOFF * std;
                let delta = (stop - start) / (np_float - 1.0);

                let mut total_weight: f64 = 0.0;

                let mut values: Vec<WeightedPoint> = (0..num_points)
                    .map(|x| {
                        let value = start + x as f64 * delta;
                        let weight = gaussian_point(mean, std, value);
                        total_weight += weight;

                        WeightedPoint { value, weight }
                    })
                    .collect();

                if !total_weight.is_finite() || total_weight <= 0.0 {
                    return Err(CrabScatError::InvalidParameter {
                        name: "gaussian weights",
                        value: total_weight,
                        reason: "total weight must be positive and finite",
                    });
                }

                for point in values.iter_mut() {
                    point.weight /= total_weight;
                }

                Ok(values)
            }

            Self::Uniform {
                start,
                stop,
                num_points,
            } => {
                let start = *start;
                let stop = *stop;
                let num_points = *num_points;
                validate_uniform(start, stop, num_points)?;
                let np_float = num_points as f64;

                let delta = (stop - start) / (np_float - 1.0);

                let values: Vec<WeightedPoint> = (0..num_points)
                    .map(|x| WeightedPoint {
                        value: start + x as f64 * delta,
                        weight: 1.0 / np_float,
                    })
                    .collect();

                Ok(values)
            }
        }
    }
}

fn validate_gaussian(mean: f64, std: f64, num_points: usize) -> Result<()> {
    if !mean.is_finite() {
        return Err(CrabScatError::InvalidParameter {
            name: "mean",
            value: mean,
            reason: "mean must be finite",
        });
    }

    if !std.is_finite() || std <= 0.0 {
        return Err(CrabScatError::InvalidParameter {
            name: "std",
            value: std,
            reason: "standard deviation must be positive and finite",
        });
    }

    validate_num_points(num_points)
}

fn validate_uniform(start: f64, stop: f64, num_points: usize) -> Result<()> {
    if !start.is_finite() {
        return Err(CrabScatError::InvalidParameter {
            name: "start",
            value: start,
            reason: "start must be finite",
        });
    }

    if !stop.is_finite() {
        return Err(CrabScatError::InvalidParameter {
            name: "stop",
            value: stop,
            reason: "stop must be finite",
        });
    }

    if stop <= start {
        return Err(CrabScatError::InvalidParameter {
            name: "stop",
            value: stop,
            reason: "stop must be greater than start",
        });
    }

    validate_num_points(num_points)
}

fn validate_num_points(num_points: usize) -> Result<()> {
    if num_points < 2 {
        return Err(CrabScatError::InvalidParameter {
            name: "num_points",
            value: num_points as f64,
            reason: "num_points must be at least 2",
        });
    }

    Ok(())
}

fn gaussian_point(mean: f64, std: f64, pos: f64) -> f64 {
    let numerator = -(pos - mean).powi(2);
    let denominator = 2.0 * std.powi(2);
    (numerator / denominator).exp()
}

#[cfg(test)]
mod tests {
    use super::Distribution;
    use crate::Result;

    #[test]
    fn uniform_weights_sum_to_one() -> Result<()> {
        let distribution = Distribution::Uniform {
            start: 10.0,
            stop: 20.0,
            num_points: 5,
        };

        let total_weight: f64 = distribution
            .sample_points()?
            .iter()
            .map(|point| point.weight)
            .sum();

        assert!((total_weight - 1.0).abs() < 1.0e-12);
        Ok(())
    }

    #[test]
    fn uniform_sampling_includes_endpoints() -> Result<()> {
        let distribution = Distribution::Uniform {
            start: 10.0,
            stop: 20.0,
            num_points: 5,
        };

        let points = distribution.sample_points()?;

        assert_eq!(points.first().expect("at least one point").value, 10.0);
        assert_eq!(points.last().expect("at least one point").value, 20.0);
        Ok(())
    }

    #[test]
    fn gaussian_weights_sum_to_one() -> Result<()> {
        let distribution = Distribution::Gaussian {
            mean: 50.0,
            std: 5.0,
            num_points: 9,
        };

        let total_weight: f64 = distribution
            .sample_points()?
            .iter()
            .map(|point| point.weight)
            .sum();

        assert!((total_weight - 1.0).abs() < 1.0e-12);
        Ok(())
    }

    #[test]
    fn gaussian_sampling_is_symmetric_around_mean() -> Result<()> {
        let distribution = Distribution::Gaussian {
            mean: 50.0,
            std: 5.0,
            num_points: 9,
        };

        let points = distribution.sample_points()?;
        let first = points.first().expect("at least one point");
        let last = points.last().expect("at least one point");

        assert!(((first.value + last.value) / 2.0 - 50.0).abs() < 1.0e-12);
        assert!((first.weight - last.weight).abs() < 1.0e-12);
        Ok(())
    }
}

use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, CrabScatError>;

#[derive(Debug)]
pub enum CrabScatError {
    EmptyProfile,
    LengthMismatch {
        expected: usize,
        found: usize,
        field: &'static str,
    },
    InvalidIntegrationInterval {
        a: f64,
        b: f64,
        reason: &'static str,
    },
    InvalidIntegrationSteps {
        n: usize,
        reason: &'static str,
    },
    InvalidParameter {
        name: &'static str,
        value: f64,
        reason: &'static str,
    },
    NonPositiveUncertainty {
        index: usize,
        value: f64,
    },
    ParseError {
        line: usize,
        message: String,
    },
    NonFiniteIntegrand {
        x: f64,
        value: f64,
    },
    NotEnoughData {
        points: usize,
        parameters: usize,
    },
    Io(std::io::Error),
}

impl fmt::Display for CrabScatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProfile => write!(f, "profile must contain at least one point"),
            Self::LengthMismatch {
                expected,
                found,
                field,
            } => {
                write!(
                    f,
                    "length mismatch for {field}: expected {expected} values, found {found}"
                )
            }
            Self::InvalidIntegrationInterval { a, b, reason } => {
                write!(f, "invalid integration interval [{a}, {b}]: {reason}")
            }
            Self::InvalidIntegrationSteps { n, reason } => {
                write!(f, "invalid integration step count {n}: {reason}")
            }
            Self::InvalidParameter {
                name,
                value,
                reason,
            } => write!(f, "invalid parameter `{name}` = {value}: {reason}"),
            Self::NonPositiveUncertainty { index, value } => write!(
                f,
                "uncertainty at index {index} must be positive and finite, got {value}"
            ),
            Self::ParseError { line, message } => {
                write!(f, "failed to parse data at line {line}: {message}")
            }
            Self::NonFiniteIntegrand { x, value } => {
                write!(f, "integrand returned non-finite value {value} at x = {x}")
            }
            Self::NotEnoughData { points, parameters } => write!(
                f,
                "not enough data points for reduced chi-squared: {points} points, {parameters} parameters"
            ),
            Self::Io(error) => write!(f, "{error}"),
        }
    }
}

impl Error for CrabScatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CrabScatError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

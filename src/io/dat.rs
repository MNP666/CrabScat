use std::fs;
use std::path::Path;

use crate::data::Profile;
use crate::error::{CrabScatError, Result};

pub fn read_dat(path: impl AsRef<Path>) -> Result<Profile> {
    let input = fs::read_to_string(path)?;
    parse_dat(&input)
}

pub fn parse_dat(input: &str) -> Result<Profile> {
    let mut q = Vec::new();
    let mut intensity = Vec::new();
    let mut sigma = Vec::new();
    let mut expected_columns: Option<usize> = None;

    for (line_index, line) in input.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let columns: Vec<&str> = trimmed.split_whitespace().collect();

        match expected_columns {
            Some(count) if count != columns.len() => {
                return Err(CrabScatError::ParseError {
                    line: line_index + 1,
                    message: format!(
                        "expected {count} columns based on earlier data lines, found {}",
                        columns.len()
                    ),
                });
            }
            None => expected_columns = Some(columns.len()),
            _ => {}
        }

        if !(2..=3).contains(&columns.len()) {
            return Err(CrabScatError::ParseError {
                line: line_index + 1,
                message: "expected two or three whitespace-separated columns".to_string(),
            });
        }

        let q_value = parse_f64(columns[0], line_index + 1, "q")?;
        let intensity_value = parse_f64(columns[1], line_index + 1, "intensity")?;

        q.push(q_value);
        intensity.push(intensity_value);

        if columns.len() == 3 {
            sigma.push(parse_f64(columns[2], line_index + 1, "sigma")?);
        }
    }

    let sigma = if expected_columns == Some(3) {
        Some(sigma)
    } else {
        None
    };

    Profile::new(q, intensity, sigma)
}

fn parse_f64(token: &str, line: usize, field: &str) -> Result<f64> {
    token.parse::<f64>().map_err(|_| CrabScatError::ParseError {
        line,
        message: format!("could not parse {field} value `{token}` as f64"),
    })
}

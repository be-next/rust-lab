use anyhow::{Context, Result};

/// Function that calculates the inverse of a number represented as a string
pub fn calculate_inverse(s: &str) -> Result<f64> {
    // Attempt to convert the string to an integer
    let n: i32 = s
        .parse()
        .context("Failed to parse the input string as an integer")?;
    if n == 0 {
        Err(anyhow::anyhow!("Division by zero"))
    } else {
        Ok(1.0 / n as f64)
    }
}

fn main() {
    match calculate_inverse("10") {
        Ok(val) => println!("The inverse is {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_inverse_nominal() {
        let result = calculate_inverse("10");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.1);
    }

    #[test]
    fn test_calculate_inverse_division_by_zero() {
        let result = calculate_inverse("0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Division by zero");
    }

    #[test]
    fn test_calculate_inverse_parse_error() {
        let result = calculate_inverse("abc");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse the input string as an integer"));
    }
}

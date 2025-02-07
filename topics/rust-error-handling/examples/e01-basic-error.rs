use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Enumeration of possible errors
#[derive(Debug)]
enum CalculationError {
    ParseError(ParseIntError),
    DivisionByZero,
}

// Implementation of Display trait for better readability
impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculationError::ParseError(e) => write!(f, "Conversion error: {}", e),
            CalculationError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

// Implementation of Error trait
impl Error for CalculationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CalculationError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

/// Function that calculates the inverse of a number represented as a string
fn calculate_inverse(s: &str) -> Result<f64, CalculationError> {
    // Attempt to convert the string to an integer
    let n: i32 = s.parse().map_err(CalculationError::ParseError)?;
    if n == 0 {
        Err(CalculationError::DivisionByZero)
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
        assert!(matches!(
            result.unwrap_err(),
            CalculationError::DivisionByZero
        ));
    }

    #[test]
    fn test_calculate_inverse_parse_error() {
        let result = calculate_inverse("abc");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CalculationError::ParseError(_)
        ));
    }
}

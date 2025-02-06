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

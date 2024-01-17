use std::fmt;
use std::error::Error;
use std::num::{ParseFloatError, ParseIntError};

// Define a custom error enum that wraps ParseFloatError and ParseIntError
#[derive(Debug)]
pub enum ParseError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
    Empty
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::IntError(err)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> Self {
        ParseError::FloatError(err)
    }
}

// Implement the Error trait for ParseError
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FloatError(err) => write!(f, "Float parsing error: {}", err),
            ParseError::IntError(err) => write!(f, "Int parsing error: {}", err),
            ParseError::Empty => write!(f,"Missing items"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_string(values : &str) -> Result<(usize, usize, f64), ParseError> {
    let mut parts = values.split_whitespace();
    parts.next();
    let first_usize: usize = parts.next()
      .ok_or(ParseError::Empty)?
      .parse()?;
    let second_usize: usize = parts.next()
      .ok_or(ParseError::Empty)?
      .parse()?;
    let float_value: f64 = parts.next()
      .ok_or(ParseError::Empty)?
      .parse()?;

    Ok((first_usize, second_usize, float_value))
}

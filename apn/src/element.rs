use crate::parser::ParserError;
use Element::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum ElementError {}

impl TryFrom<&str> for Element {
    type Error = ParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(integer) = value.parse::<i64>() {
            Ok(Integer(integer))
        } else if let Ok(float) = value.parse::<f64>() {
            Ok(Float(float))
        } else {
            Err(ParserError::InvalidElement(value.to_string()))
        }
    }
}
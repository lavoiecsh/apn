use crate::parser::ParserError;
use Element::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Integer(i64),
    Float(f64),
}

impl Element {
    pub(crate) fn add(self, other: Element) -> Result<Element, ElementError> {
        match (self, other) {
            (Integer(lhs), Integer(rhs)) => Ok(Integer(lhs + rhs)),
            (Float(lhs), Float(rhs)) => Ok(Float(lhs + rhs)),
            (Integer(lhs), Float(rhs)) => Ok(Float(lhs as f64 + rhs)),
            (Float(lhs), Integer(rhs)) => Ok(Float(lhs + rhs as f64)),
        }
    }

    pub(crate) fn subtract(self, other: Element) -> Result<Element, ElementError> {
        match (self, other) {
            (Integer(lhs), Integer(rhs)) => Ok(Integer(lhs - rhs)),
            (Float(lhs), Float(rhs)) => Ok(Float(lhs - rhs)),
            (Integer(lhs), Float(rhs)) => Ok(Float(lhs as f64 - rhs)),
            (Float(lhs), Integer(rhs)) => Ok(Float(lhs - rhs as f64)),
        }
    }
}

#[derive(Debug)]
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
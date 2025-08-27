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

    pub(crate) fn multiply(self, other: Element) -> Result<Element, ElementError> {
        match (self, other) {
            (Integer(lhs), Integer(rhs)) => Ok(Integer(lhs * rhs)),
            (Float(lhs), Float(rhs)) => Ok(Float(lhs * rhs)),
            (Integer(lhs), Float(rhs)) => Ok(Float(lhs as f64 * rhs)),
            (Float(lhs), Integer(rhs)) => Ok(Float(lhs * rhs as f64)),
        }
    }

    pub(crate) fn divide(self, other: Element) -> Result<Element, ElementError> {
        match (self, other) {
            (Integer(lhs), Integer(rhs)) => Ok(Float(lhs as f64 / rhs as f64)),
            (Float(lhs), Float(rhs)) => Ok(Float(lhs / rhs)),
            (Integer(lhs), Float(rhs)) => Ok(Float(lhs as f64 / rhs)),
            (Float(lhs), Integer(rhs)) => Ok(Float(lhs / rhs as f64)),
        }
    }

    pub(crate) fn modulo(self, other: Element) -> Result<Element, ElementError> {
        match (self, other) {
            (Integer(lhs), Integer(rhs)) => Ok(Integer(lhs % rhs)),
            (Float(lhs), Float(rhs)) => Ok(Float(lhs % rhs)),
            (Integer(lhs), Float(rhs)) => Ok(Float(lhs as f64 % rhs)),
            (Float(lhs), Integer(rhs)) => Ok(Float(lhs % rhs as f64)),
        }
    }

    pub(crate) fn floor(self) -> Result<Element, ElementError> {
        match self {
            Integer(value) => Ok(Integer(value)),
            Float(value) => Ok(Integer(value as i64)),
        }
    }

    pub(crate) fn ceiling(self) -> Result<Element, ElementError> {
        match self {
            Integer(value) => Ok(Integer(value)),
            Float(value) => {
                if value - (value as i64) as f64 == 0.0 {
                    Ok(Integer(value as i64 + 1))
                } else {
                    Ok(Integer(value as i64))
                }
            }
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
use crate::element::Element;
use crate::environment::{Environment, EvaluationError};
use crate::function::*;

pub(crate) fn parse(input: impl Into<String>) -> Result<Vec<Token>, ParserError> {
    input.into()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(Token::try_from)
        .collect()
}

#[derive(Debug)]
pub enum Token {
    Element(Element),
    Function(fn(&mut Environment) -> Result<(), EvaluationError>),
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidElement(String),
    InvalidToken(String),
}

impl TryFrom<&str> for Token {
    type Error = ParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" | "+" => Ok(Token::Function(add)),
            "subtract" | "-" => Ok(Token::Function(subtract)),
            _ => {
                if let Ok(element) = Element::try_from(value) {
                    Ok(Token::Element(element))
                } else {
                    Err(ParserError::InvalidToken(value.to_string()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;
    use std::ptr::fn_addr_eq;

    macro_rules! fn_eq {
        ($l: tt, $r: tt) => {
            fn_addr_eq($l, $r as for<'a> fn(&'a mut Environment) -> Result<(), EvaluationError>)
        }
    }

    #[test]
    fn parses_number_as_element() {
        let result = parse("1");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_matches!(tokens[0], Token::Element(Element::Integer(1)));
    }

    #[test]
    fn parses_multiple_numbers_as_elements() {
        let result = parse("1 2 3");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 3);
        assert_matches!(tokens[0], Token::Element(Element::Integer(1)));
        assert_matches!(tokens[1], Token::Element(Element::Integer(2)));
        assert_matches!(tokens[2], Token::Element(Element::Integer(3)));
    }

    #[test]
    fn parses_function_names_as_function_calls() {
        let result = parse("1 2 add");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 3);
        assert_matches!(tokens[2], Token::Function(f) if fn_eq!(f, add));
    }

    #[test]
    fn parses_function_symbols_as_function_calls() {
        let result = parse("1 2 +");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 3);
        assert_matches!(tokens[2], Token::Function(f) if fn_eq!(f, add));
    }
}

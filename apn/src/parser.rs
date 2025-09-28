use crate::element::Element;
use crate::function::Function;

pub(super) fn parse(input: impl Into<String>) -> Result<Vec<Element>, ParserError> {
    let tokens = input
        .into()
        .replace('[', " [ ")
        .replace(']', " ] ")
        .split(" ")
        .filter(|t| !t.is_empty())
        .map(|s| Token::try_from(s))
        .collect::<Result<Vec<Token>, ParserError>>()?;
    let mut parser = Parser::new(&tokens);
    parser.parse()?;
    Ok(parser.elements)
}

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    elements: Vec<Element>,
    depth: usize,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            elements: Vec::new(),
            depth: 0,
            index: 0,
        }
    }

    fn parse(&mut self) -> Result<usize, ParserError> {
        while self.index < self.tokens.len() {
            match &self.tokens[self.index] {
                Token::Element(element) => {
                    self.elements.push(element.clone());
                }
                Token::Function(function) => {
                    self.elements.push(Element::Function(function.clone()));
                }
                Token::ArrayStart => {
                    let mut child = Self {
                        tokens: self.tokens,
                        elements: Vec::new(),
                        depth: self.depth + 1,
                        index: self.index + 1,
                    };
                    self.index = child.parse()?;
                    self.elements.push(Element::Array(child.elements));
                }
                Token::ArrayEnd => {
                    return if self.depth == 0 {
                        Err(ParserError::NotInsideArray)
                    } else {
                        Ok(self.index)
                    };
                }
            }
            self.index += 1;
        }
        if self.depth == 0 {
            Ok(self.index)
        } else {
            Err(ParserError::UnterminatedArray)
        }
    }
}

enum Token {
    Element(Element),
    Function(Function),
    ArrayStart,
    ArrayEnd,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidToken(String),
    UnterminatedArray,
    NotInsideArray,
}

impl TryFrom<&str> for Token {
    type Error = ParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "[" => Ok(Token::ArrayStart),
            "]" => Ok(Token::ArrayEnd),
            _ => {
                if let Ok(function) = Function::try_from(value) {
                    Ok(Token::Function(function))
                } else if let Ok(element) = Element::try_from(value) {
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

    #[test]
    fn parses_number_as_element() {
        let result = parse("1");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_matches!(tokens[0], Element::Integer(1));
    }

    #[test]
    fn parses_multiple_numbers_as_elements() {
        let result = parse("1 2 3");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 3);
        assert_matches!(tokens[0], Element::Integer(1));
        assert_matches!(tokens[1], Element::Integer(2));
        assert_matches!(tokens[2], Element::Integer(3));
    }

    #[test]
    fn parses_arrays() {
        let result = parse("[1 2 3]");
        assert_matches!(result, Ok(_));
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_matches!(&tokens[0], Element::Array(array) if array == &vec![Element::Integer(1), Element::Integer(2), Element::Integer(3)]);
    }

    #[test]
    fn parses_complex_array() {
        let result = parse("1 [ 2 + ] 3 [ 4 [ 5 ] ]");
        assert_matches!(result, Ok(_));
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 4);
        assert_matches!(&tokens[0], Element::Integer(1));
        assert_matches!(&tokens[1], Element::Array(array) if array.len() == 2);
        if let Element::Array(array) = &tokens[1] {
            assert_matches!(&array[0], Element::Integer(2));
            assert_matches!(&array[1], Element::Function(_));
        }
        assert_matches!(&tokens[2], Element::Integer(3));
        assert_matches!(&tokens[3], Element::Array(array) if array.len() == 2);
        if let Element::Array(array) = &tokens[3] {
            assert_matches!(&array[0], Element::Integer(4));
            assert_matches!(&array[1], Element::Array(sub_array) if sub_array.len() == 1);
            if let Element::Array(sub_array) = &array[1] {
                assert_matches!(&sub_array[0], Element::Integer(5));
            }
        }
    }
}

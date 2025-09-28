use crate::function::Function;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Variable(String),
    Function(Function),
    Array(Vec<Element>),
}

#[derive(Debug, PartialEq)]
pub enum ElementError {}

impl TryFrom<&str> for Element {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "true" {
            return Ok(Element::Boolean(true));
        }
        if value == "false" {
            return Ok(Element::Boolean(false));
        }
        let s = value.split('$').collect::<Vec<&str>>();
        if s.len() == 2 {
            return Ok(Element::Variable(s[1].into()));
        }
        if let Ok(function) = Function::try_from(value) {
            return Ok(Element::Function(function));
        }
        if let Ok(integer) = value.parse::<i64>() {
            return Ok(Element::Integer(integer));
        }
        if let Ok(float) = value.parse::<f64>() {
            return Ok(Element::Float(float));
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn parses_true_as_boolean() {
        assert_matches!(Element::try_from("true"), Ok(Element::Boolean(true)));
    }

    #[test]
    fn parses_false_as_boolean() {
        assert_matches!(Element::try_from("false"), Ok(Element::Boolean(false)));
    }

    #[test]
    fn parses_dollar_as_variable() {
        assert_matches!(Element::try_from("$some_var_name"), Ok(Element::Variable(name)) if name == String::from("some_var_name"));
    }

    #[test]
    fn parses_integer_as_integer() {
        assert_matches!(Element::try_from("5"), Ok(Element::Integer(5)));
    }

    #[test]
    fn parses_float_as_float() {
        assert_matches!(Element::try_from("3.14"), Ok(Element::Float(3.14)));
        assert_matches!(Element::try_from("1."), Ok(Element::Float(1.)));
        assert_matches!(Element::try_from("1e2"), Ok(Element::Float(1e2)));
    }
}
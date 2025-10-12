use crate::function::Function;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    Variable(String),
    Function(Function),
    Array(Vec<Element>),
}

impl Element {
    pub(crate) fn is_string(&self) -> bool {
        if let Element::Array(elements) = self {
            elements.iter().all(|e| matches!(e, Element::Char(_)))
        } else {
            false
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Boolean(true) => f.write_str("true"),
            Element::Boolean(false) => f.write_str("false"),
            Element::Integer(i) => write!(f, "{}", i),
            Element::Float(fl) => write!(f, "{}", fl),
            Element::Char(c) => write!(f, "'{}'", c),
            Element::Variable(v) => write!(f, "${}", v),
            Element::Function(fu) => write!(f, "f({})", fu.name()),
            Element::Array(elements) => {
                if self.is_string() {
                    write!(
                        f,
                        "\"{}\"",
                        elements
                            .iter()
                            .map(|e| match e {
                                Element::Char(c) => c.to_string(),
                                _ => unreachable!("expecting char"),
                            })
                            .collect::<String>()
                    )
                } else {
                    f.write_str("[ ")?;
                    for e in elements {
                        write!(f, "{} ", e)?;
                    }
                    f.write_str("]")
                }
            }
        }
    }
}

impl TryFrom<&str> for Element {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "true" {
            return Ok(Element::Boolean(true));
        }
        if value == "false" {
            return Ok(Element::Boolean(false));
        }
        // let chars = value.chars().collect::<Vec<char>>();
        // let chars_len = chars.len();
        // if chars[0] == '$' {
        //     return Ok(Element::Variable(chars.iter().skip(1).collect::<String>()));
        // }
        // if chars_len == 3 && chars[0] == '\'' && chars[2] == '\'' {
        //     return Ok(Element::Char(chars[1]));
        // }
        // if chars[0] == '"' && chars[chars.len() - 1] == '"' {
        //     return Ok(Element::Array(
        //         chars
        //             .into_iter()
        //             .skip(1)
        //             .take(chars_len - 2)
        //             .map(Element::Char)
        //             .collect(),
        //     ));
        // }
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

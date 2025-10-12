use crate::element::Element;

pub(super) fn parse(input: impl Into<String>) -> Result<Vec<Element>, ParserError> {
    let chars = input.into().chars().collect::<Vec<char>>();
    Ok(parse_chars(&chars, 0, 0)?.0)
}

fn parse_chars(
    chars: &[char],
    depth: usize,
    index: usize,
) -> Result<(Vec<Element>, usize), ParserError> {
    let mut elements = Vec::new();
    let mut index = index;
    while index < chars.len() {
        match chars[index] {
            '"' => {
                let (string, new_index) = read_string(chars, index + 1)?;
                elements.push(Element::Array(string));
                index = new_index;
            }
            '\'' => {
                if index + 2 >= chars.len() {
                    return Err(ParserError::EndOfInput);
                }
                if chars[index + 2] != '\'' {
                    return Err(ParserError::InvalidToken(
                        chars[index..index + 3].iter().collect(),
                    ));
                }
                elements.push(Element::Char(chars[index + 1]));
                index += 3;
            }
            '$' => {
                let (element, new_index) = read_variable(chars, index + 1)?;
                elements.push(element);
                index = new_index;
            }
            '[' => {
                let (array, new_index) = parse_chars(chars, depth + 1, index + 1)?;
                elements.push(Element::Array(array));
                index = new_index;
            }
            ']' => {
                return if depth == 0 {
                    Err(ParserError::NotInsideArray)
                } else {
                    Ok((elements, index + 1))
                };
            }
            ' ' | '\t' | '\n' => {
                index += 1;
            }
            _ => {
                let (element, new_index) = read_element(chars, index)?;
                elements.push(element);
                index = new_index;
            }
        }
    }
    Ok((elements, index))
}

fn read_string(chars: &[char], index: usize) -> Result<(Vec<Element>, usize), ParserError> {
    let mut max_index = index;
    let mut escaped = false;
    while max_index < chars.len() {
        match (chars[max_index], escaped) {
            ('\\', false) => {
                escaped = true;
                max_index += 1;
            }
            ('"', false) => {
                break;
            }
            _ => {
                max_index += 1;
                escaped = false;
            }
        }
    }
    Ok((chars[index..max_index].iter().cloned().map(Element::Char).collect(), max_index + 1))
}

fn read_variable(chars: &[char], index: usize) -> Result<(Element, usize), ParserError> {
    let mut max_index = index;
    while max_index < chars.len() {
        match chars[max_index] {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => max_index += 1,
            _ => break,
        }
    }
    Ok((Element::Variable(chars[index..max_index].iter().collect()), max_index))
}

fn read_element(chars: &[char], index: usize) -> Result<(Element, usize), ParserError> {
    let mut max_index = index;
    while max_index < chars.len() {
        match chars[max_index] {
            ' ' | '\t' | '\n' | '\\' | '[' | ']' | '"' | '$' | '\'' => break,
            _ => max_index += 1,
        }
    }
    if let Ok(element) = Element::try_from(chars[index..max_index].iter().collect::<String>().as_str()) {
        Ok((element, max_index))
    } else {
        Err(ParserError::EndOfInput)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidToken(String),
    UnterminatedArray,
    UnterminatedString,
    NotInsideArray,
    UnknownCharacter,
    EndOfInput,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn parses_number_as_element() {
        let result = parse("1");
        assert!(result.is_ok());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 1);
        assert_matches!(elements[0], Element::Integer(1));
    }

    #[test]
    fn parses_multiple_numbers_as_elements() {
        let result = parse("1 2 3");
        assert!(result.is_ok());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 3);
        assert_matches!(elements[0], Element::Integer(1));
        assert_matches!(elements[1], Element::Integer(2));
        assert_matches!(elements[2], Element::Integer(3));
    }

    #[test]
    fn parses_arrays() {
        let result = parse("[1 2 3]");
        assert_matches!(result, Ok(_));
        let elements = result.unwrap();
        assert_eq!(elements.len(), 1);
        assert_matches!(&elements[0], Element::Array(array) if array == &vec![Element::Integer(1), Element::Integer(2), Element::Integer(3)]);
    }

    #[test]
    fn parses_complex_array() {
        let result = parse("1 [ 2 + ] 3 [ 4 [ 5 ] ]");
        assert_matches!(result, Ok(_));
        let elements = result.unwrap();
        assert_eq!(elements.len(), 4);
        assert_matches!(&elements[0], Element::Integer(1));
        assert_matches!(&elements[1], Element::Array(array) if array.len() == 2);
        if let Element::Array(array) = &elements[1] {
            assert_matches!(&array[0], Element::Integer(2));
            assert_matches!(&array[1], Element::Function(_));
        }
        assert_matches!(&elements[2], Element::Integer(3));
        assert_matches!(&elements[3], Element::Array(array) if array.len() == 2);
        if let Element::Array(array) = &elements[3] {
            assert_matches!(&array[0], Element::Integer(4));
            assert_matches!(&array[1], Element::Array(sub_array) if sub_array.len() == 1);
            if let Element::Array(sub_array) = &array[1] {
                assert_matches!(&sub_array[0], Element::Integer(5));
            }
        }
    }

    #[test]
    fn parses_variables_functions_arrays() {
        let result = parse("[1 +] $inc = 2 [$inc ] . .");
        assert_matches!(result, Ok(_));
        let elements = result.unwrap();
        assert_eq!(elements.len(), 7);
        assert_matches!(&elements[0], Element::Array(_));
        assert_matches!(&elements[1], Element::Variable(name) if name == "inc");
        assert_matches!(&elements[2], Element::Function(f) if f.name() == "=");
        assert_matches!(&elements[3], Element::Integer(2));
        assert_matches!(&elements[4], Element::Array(_));
        assert_matches!(&elements[5], Element::Function(f) if f.name() == ".");
        assert_matches!(&elements[6], Element::Function(f) if f.name() == ".");
    }
}

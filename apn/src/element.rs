#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Boolean(bool),
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum ElementError {}

impl TryFrom<&str> for Element {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "true" {
            Ok(Element::Boolean(true))
        } else if value == "false" {
            Ok(Element::Boolean(false))
        } else if let Ok(integer) = value.parse::<i64>() {
            Ok(Element::Integer(integer))
        } else if let Ok(float) = value.parse::<f64>() {
            Ok(Element::Float(float))
        } else {
            Err(())
        }
    }
}
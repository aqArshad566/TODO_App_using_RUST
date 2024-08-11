use std::{fmt, num::{IntErrorKind, ParseIntError}};

#[derive(Debug)]
pub enum TodoError {
    InvalidInput(ParseIntError),
    TodoNotFound,
    EmptyList,
}

impl TodoError {
    pub fn err(&self) -> String {
        match self {
            TodoError::EmptyList => format!("EmptyList::{{The to-do list is empty}}"),
            TodoError::TodoNotFound => format!("TodoNotFound::{{Selected to-do is not found in the list}}"),
            // TodoError::InvalidInput => format!("InvalidInput::{{The input entered is Invalid!}}"),
            TodoError::InvalidInput(e) => 
                match e.kind() {
                    IntErrorKind::InvalidDigit => format!("InvalidInput::{{The input entered is not a +ve Integer}}"),
                    IntErrorKind::NegOverflow => format!("NegativeInteger::{{Negative number not allowed}}"),
                    IntErrorKind::Empty => format!("EmptyInput::{{Input not entered}}"),
                    _ => "".to_string(),
                },
        }
    }
}


impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err())
    }
}


impl std::error::Error for TodoError { }


use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

impl Display for Mode {
    fn fmt(self: &Self, f: &mut Formatter) -> Result {
        let str = match self {
            Mode::Normal => "normal",
            Mode::Insert => "insert",
            Mode::Visual => "visual",
        };
        write!(f, "{}", str)
    }
}

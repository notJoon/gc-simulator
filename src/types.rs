use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TriColor {
    White,
    Gray,
    Black,
}

impl fmt::Display for TriColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TriColor::White => write!(f, "White: Not visited"),
            TriColor::Gray => write!(f, "Gray: Visiting"),
            TriColor::Black => write!(f, "Black: Visited"),
        }
    }
}
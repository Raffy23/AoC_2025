pub mod utils;

mod days;
pub use days::*;

pub const INPUT_FOLDER: &'static str = "./input";

pub enum Part {
    Example,
    Part1,
    Part2,
}

impl Part {
    fn suffix(self) -> &'static str {
        match self {
            Part::Example => "example",
            Part::Part1 => "01",
            Part::Part2 => "02",
        }
    }
}

impl From<u8> for Part {
    fn from(value: u8) -> Self {
        match value {
            0 => Part::Example,
            1 => Part::Part1,
            2 => Part::Part2,
            _ => panic!("Not a valid Part"),
        }
    }
}

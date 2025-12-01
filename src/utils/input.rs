use std::fs::read_to_string;
use std::io::Result;

use crate::{INPUT_FOLDER, Part};

pub fn read_input(day: u8, part: Option<Part>) -> Result<String> {
    read_to_string(match part {
        Some(part) => format!("{}/day{:0>2}_{}.txt", INPUT_FOLDER, day, part.suffix()),
        None => format!("{}/day{:0>2}.txt", INPUT_FOLDER, day),
    })
}

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod d1;
pub mod d2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    d2::part_two()
}

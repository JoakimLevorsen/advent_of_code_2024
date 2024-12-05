#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    d4::part_two()
}

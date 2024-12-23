#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod d1;
pub mod d10;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    d14::part_one()
}

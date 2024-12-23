const INPUT: &str = "./src/d14/input.txt";

use std::iter::Peekable;

enum Quadrant {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    dx: i8,
    dy: i8,
}

impl Vector {
    pub fn parse(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<Self, &'static str> {
        parse_set(input, parse_i8).map(|(dx, dy)| Vector { dx, dy })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position<const LIMIT_X: u8, const LIMIT_Y: u8> {
    x: u8,
    y: u8,
}

fn apply_limit<const LIMIT: u8>(value: u8) -> Option<u8> {
    if value < LIMIT {
        Some(value)
    } else {
        None
    }
}

impl<const LIMIT_X: u8, const LIMIT_Y: u8> Position<LIMIT_X, LIMIT_Y> {
    pub fn add(self, vector: Vector) -> Self {
        let x = self
            .x
            .checked_add_signed(vector.dx)
            .and_then(apply_limit::<LIMIT_X>)
            .unwrap_or_else(|| {
                let mut x = i16::from(self.x) + i16::from(vector.dx);
                while x < 0 {
                    x += i16::from(LIMIT_X);
                }
                while x > i16::from(LIMIT_X) {
                    x -= i16::from(LIMIT_X);
                }
                x.try_into().unwrap()
            });
        let y = self
            .y
            .checked_add_signed(vector.dy)
            .and_then(apply_limit::<LIMIT_Y>)
            .unwrap_or_else(|| {
                let mut y = i16::from(self.y) + i16::from(vector.dy);
                while y < 0 {
                    y += i16::from(LIMIT_Y);
                }
                while y > i16::from(LIMIT_Y) {
                    y -= i16::from(LIMIT_Y);
                }
                y.try_into().unwrap()
            });
        Position { x, y }
    }

    pub fn parse(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<Self, &'static str> {
        parse_set(input, parse_u8).map(|(mut x, mut y)| {
            while x > LIMIT_X {
                x -= LIMIT_X;
            }
            while y > LIMIT_Y {
                y -= LIMIT_Y;
            }
            Position { x, y }
        })
    }

    pub const fn quadrant(self) -> Option<Quadrant> {
        let mid_x = LIMIT_X / 2;
        let mid_y = LIMIT_Y / 2;

        if self.x == mid_x || self.y == mid_y {
            return None;
        }

        Some(if self.x < mid_x {
            if self.y < mid_y {
                Quadrant::One
            } else {
                Quadrant::Four
            }
        } else if self.y < mid_y {
            Quadrant::Two
        } else {
            Quadrant::Three
        })
    }
}

struct Robot<const LIMIT_X: u8, const LIMIT_Y: u8> {
    starting: Position<LIMIT_X, LIMIT_Y>,
    direction: Vector,
}

fn parse_u8(iter: &mut Peekable<impl Iterator<Item = char>>) -> Result<u8, &'static str> {
    let first = match iter.peek().copied().ok_or("Found no numbers in string")? {
        v @ '0'..='9' => v as u8 - b'0',
        _ => return Err("Expected input to be between 0..=9"),
    };
    iter.next();

    let second = match iter.peek().copied() {
        Some(v @ '0'..='9') => v as u8 - b'0',
        _ => return Ok(first),
    };
    iter.next();

    let value = first * 10 + second;

    let third = match iter.peek().copied() {
        Some(v @ '0'..='9') => v as u8 - b'0',
        _ => return Ok(first),
    };
    iter.next();

    value
        .checked_mul(10)
        .and_then(|value| value.checked_add(third))
        .ok_or("Expected only values 0..=255")
}

fn parse_i8(iter: &mut Peekable<impl Iterator<Item = char>>) -> Result<i8, &'static str> {
    let negative = if iter.peek().ok_or("Expected string to be longer than 0")? == &'-' {
        iter.next();
        true
    } else {
        false
    };

    let number: i16 = match parse_u8(iter) {
        Ok(v) => v.into(),
        Err("Expected only values 0..=255") => return Err("Expected only values -128..=127"),
        Err(e) => return Err(e),
    };

    let number = if negative { -number } else { number };

    number.try_into().map_err(|_| "Expected values -128..=127")
}

fn parse_set<Int, I: Iterator<Item = char>>(
    iter: &mut Peekable<I>,
    parser: impl Fn(&mut Peekable<I>) -> Result<Int, &'static str>,
) -> Result<(Int, Int), &'static str> {
    let first = parser(iter)?;

    if iter.peek().copied() != Some(',') {
        return Err("Expected separator");
    }
    iter.next();

    let second = parser(iter)?;

    Ok((first, second))
}

impl<const LIMIT_X: u8, const LIMIT_Y: u8> Robot<LIMIT_X, LIMIT_Y> {
    pub fn try_parse(input: &str) -> Result<Self, &'static str> {
        let mut input = input.chars().peekable();

        if input.next() != Some('p') {
            return Err("Expected bot description to start with p");
        }

        if input.next() != Some('=') {
            return Err("Expected p to be followed by =");
        }

        let start = Position::parse(&mut input)?;

        for char in [' ', 'v', '='] {
            if input.next() != Some(char) {
                return Err("Expected ' v=' after position");
            }
        }

        let vector = Vector::parse(&mut input)?;

        Ok(Self {
            starting: start,
            direction: vector,
        })
    }

    pub fn simulate(&self, iterations: usize) -> Position<LIMIT_X, LIMIT_Y> {
        let mut end = self.starting;
        for _ in 0..iterations {
            end = end.add(self.direction);
        }
        end
    }
}

#[test]
fn test_part_one() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut four = 0;

    for line in input.lines() {
        let robot = Robot::<11, 7>::try_parse(line).unwrap();

        let end = robot.simulate(101);

        println!("Ended at {end:?}");

        let Some(quadrant) = end.quadrant() else {
            continue;
        };

        *match quadrant {
            Quadrant::One => &mut one,
            Quadrant::Two => &mut two,
            Quadrant::Three => &mut three,
            Quadrant::Four => &mut four,
        } += 1;
    }

    let product = one * two * three * four;

    assert_eq!(product, 12);
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut four = 0;

    for line in input.lines() {
        let robot = Robot::<101, 103>::try_parse(line).unwrap();

        let end = robot.simulate(101);

        println!("Ended at {end:?}");

        let Some(quadrant) = end.quadrant() else {
            continue;
        };

        *match quadrant {
            Quadrant::One => &mut one,
            Quadrant::Two => &mut two,
            Quadrant::Three => &mut three,
            Quadrant::Four => &mut four,
        } += 1;
    }

    let product = one * two * three * four;

    println!("Product is {product}");

    Ok(())
}

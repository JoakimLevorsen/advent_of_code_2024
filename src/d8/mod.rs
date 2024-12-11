const INPUT: &str = "./src/d8/input.txt";

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Vector {
    dx: i8,
    dy: i8,
}

impl Vector {
    pub const fn reverse(self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn vector_to(self, other: Position) -> Vector {
        let dx = i16::from(self.x) - i16::from(other.x);
        let dy = i16::from(self.y) - i16::from(other.y);
        Vector {
            dx: dx.try_into().expect("Input is only 50x50"),
            dy: dy.try_into().expect("Input is only 50x50"),
        }
    }

    pub fn filter_to(self, height: u8, width: u8) -> Option<Position> {
        if self.x < width && self.y < height {
            Some(self)
        } else {
            None
        }
    }

    pub fn add(self, vector: Vector) -> Option<Position> {
        let x = self.x.checked_add_signed(vector.dx)?;
        let y = self.y.checked_add_signed(vector.dy)?;
        Some(Position { x, y })
    }

    pub fn sub(self, vector: Vector) -> Option<Position> {
        let x = self.x.checked_add_signed(-vector.dx)?;
        let y = self.y.checked_add_signed(-vector.dy)?;
        Some(Position { x, y })
    }

    pub fn antipodes_with(
        self,
        other: Position,
        height: u8,
        width: u8,
    ) -> impl Iterator<Item = Position> {
        let delta = self.vector_to(other);

        [
            self.add(delta),
            self.sub(delta),
            other.add(delta),
            other.sub(delta),
        ]
        .into_iter()
        .flatten()
        .filter_map(move |position| position.filter_to(height, width))
        .filter(move |position| *position != self && *position != other)
    }

    fn iter(self, direction: Vector, height: u8, width: u8) -> PositionIterator {
        PositionIterator {
            initial: Some(self),
            last_position: self,
            direction,
            width,
            height,
        }
    }

    pub fn infinite_antipodes_with(
        self,
        other: Position,
        height: u8,
        width: u8,
    ) -> impl Iterator<Item = Position> {
        let delta = self.vector_to(other);

        // First we run to the last valid direction one way, so we can go back the other and find all valid positions

        let last_valid = self.iter(delta, height, width).last().unwrap_or(self);

        last_valid.iter(delta.reverse(), height, width)
    }
}

struct PositionIterator {
    initial: Option<Position>,
    last_position: Position,
    direction: Vector,
    width: u8,
    height: u8,
}

impl Iterator for PositionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first) = self.initial.take() {
            return Some(first);
        }

        let candidate = self.last_position.add(self.direction)?;

        let next = candidate.filter_to(self.height, self.width)?;

        self.last_position = next;

        Some(next)
    }
}

#[test]
fn test_infinite_antipodes() {
    let a = Position { x: 1, y: 1 };
    let b = Position { x: 2, y: 2 };
    for (found, i) in a.infinite_antipodes_with(b, 10, 10).zip((0..1).chain(3..)) {
        assert_eq!(found.x, i);
        assert_eq!(found.y, i);
    }

    let a = Position { x: 3, y: 3 };
    let b = Position { x: 4, y: 4 };
    let expected = (0..=2).chain(5..);
    for (found, expected) in a.infinite_antipodes_with(b, 10, 10).zip(expected) {
        assert_eq!(found.x, expected);
        assert_eq!(found.y, expected);
    }

    let a = Position { x: 6, y: 5 };
    let b = Position { x: 9, y: 9 };
    let expected = [
        Position { x: 3, y: 1 },
        Position { x: 6, y: 5 },
        Position { x: 9, y: 9 },
    ];
    for (found, expected) in a.infinite_antipodes_with(b, 12, 12).zip(expected) {
        assert_eq!(expected, found);
        println!("{found:?} did match {expected:?}");
    }
}

fn find_positions(input: &str) -> impl IntoIterator<Item = (u8, Vec<Position>)> {
    let mut map: HashMap<u8, Vec<Position>> = HashMap::new();

    for (line, y) in input.lines().zip(0..) {
        for (char, x) in line.bytes().zip(0..) {
            if char == b'.' {
                continue;
            }

            let position = Position { x, y };
            map.entry(char)
                .and_modify(|vec| vec.push(position))
                .or_insert_with(|| vec![position]);
        }
    }

    map.into_iter()
}

struct IterWithRemaining<I>
where
    I: Iterator + Clone,
{
    iter: I,
}

impl<I> IterWithRemaining<I>
where
    I: Iterator + Clone,
{
    fn new(iter: I) -> Self {
        IterWithRemaining { iter }
    }
}

impl<I> Iterator for IterWithRemaining<I>
where
    I: Iterator + Clone,
{
    type Item = (I::Item, I);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| (item, self.iter.clone()))
    }
}

fn find_antipodes<'a>(
    positions: impl Iterator<Item = Position> + Clone + 'a,
    height: u8,
    width: u8,
) -> impl Iterator<Item = Position> + 'a {
    IterWithRemaining::new(positions).flat_map(move |(start, remaining)| {
        remaining.flat_map(move |end| start.antipodes_with(end, height, width))
    })
}

#[test]
fn test_part_one() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let mut antipodes = HashSet::new();

    for (_, positions) in find_positions(input) {
        for antipode in find_antipodes(positions.into_iter(), 12, 12) {
            antipodes.insert(antipode);
        }
    }

    assert_eq!(antipodes.len(), 14);
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let mut antipodes = HashSet::new();

    for (_, positions) in find_positions(&input) {
        for antipode in find_antipodes(positions.into_iter(), 50, 50) {
            antipodes.insert(antipode);
        }
    }

    println!("Found {} antipodes", antipodes.len());

    Ok(())
}

fn find_all_antipodes<'a>(
    positions: impl Iterator<Item = Position> + Clone + 'a,
    height: u8,
    width: u8,
) -> impl Iterator<Item = Position> + 'a {
    IterWithRemaining::new(positions).flat_map(move |(start, remaining)| {
        remaining.flat_map(move |end| start.infinite_antipodes_with(end, height, width))
    })
}

#[test]
fn test_part_two() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let expected: Vec<&'static str> = "##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##"
        .lines()
        .collect();

    let mut antipodes = HashSet::new();

    for (char, positions) in find_positions(input) {
        for antipode in find_all_antipodes(positions.into_iter(), 12, 12) {
            let in_expected = expected[usize::from(antipode.y)].as_bytes()[usize::from(antipode.x)];
            let char = char::from_u32(char as u32).unwrap();
            assert!(
                in_expected != b'.',
                "Should not have generated {antipode:?} for '{char}'"
            );
            antipodes.insert(antipode);
        }
    }

    assert_eq!(antipodes.len(), 34);
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let mut antipodes = HashSet::new();

    for (_, positions) in find_positions(&input) {
        for antipode in find_all_antipodes(positions.into_iter(), 50, 50) {
            antipodes.insert(antipode);
        }
    }

    println!("Found {} antipodes", antipodes.len());

    Ok(())
}

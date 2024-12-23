const INPUT: &str = "./src/d6/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
}

type Map = Vec<Vec<Cell>>;

fn parse_map(input: &str) -> (Map, Position) {
    let mut map = Vec::new();
    let mut guard = None;

    for (y, line) in input.lines().enumerate() {
        let mut map_line = Vec::with_capacity(line.as_bytes().len());

        for (x, cell) in line.as_bytes().iter().enumerate() {
            map_line.push(match cell {
                b'.' => Cell::Empty,
                b'#' => Cell::Obstacle,
                b'^' => {
                    guard = Some(Position { x, y });
                    Cell::Empty
                }
                unknown => panic!("Saw unknown char {unknown}"),
            });
        }

        map.push(map_line);
    }

    (map, guard.expect("Should have seen guard on map"))
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn advance(&self, direction: Direction, width: usize, height: usize) -> Option<Position> {
        let (dx, dy) = match direction {
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
        };

        let x = self.x.checked_add_signed(dx)?;

        if x >= width {
            return None;
        }

        let y = self.y.checked_add_signed(dy)?;

        if y >= height {
            return None;
        }

        Some(Position { x, y })
    }
}

fn run_sim(input: &str) -> usize {
    let (map, mut guard_position) = parse_map(input);
    let height = map.len();
    let width = map[0].len();
    let mut direction = Direction::Up;

    let mut visited = std::collections::HashSet::new();

    loop {
        let Some(next) = guard_position.advance(direction, width, height) else {
            break;
        };

        if map[next.y][next.x] == Cell::Obstacle {
            direction = direction.turn();
            continue;
        }

        guard_position = next;
        visited.insert(next);
    }

    visited.len()
}

#[test]
fn run_test_sim() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    assert_eq!(run_sim(input), 41);
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let unique = run_sim(&input);

    println!("Ran {unique} unique steps");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellContent {
    None,
    Wall,
    Box,
    Robot,
}

impl std::fmt::Display for CellContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellContent::None => ' ',
                CellContent::Wall => '#',
                CellContent::Box => 'O',
                CellContent::Robot => 'R',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const fn vector(self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'V',
                Direction::Left => '<',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Position<const LIMIT_X: usize, const LIMIT_Y: usize> {
    x: u8,
    y: u8,
}

impl<const LIMIT_X: usize, const LIMIT_Y: usize> Position<LIMIT_X, LIMIT_Y> {
    #[must_use]
    pub const fn new(x: u8, y: u8) -> Self {
        assert!((x as usize) < LIMIT_X);
        assert!((y as usize) < LIMIT_Y);
        Self { x, y }
    }

    pub fn neighbor(self, direction: Direction) -> Option<Self> {
        let vector = direction.vector();
        let x = self.x.checked_add_signed(vector.0)?;
        let y = self.y.checked_add_signed(vector.1)?;

        Position::try_from((x, y)).ok()
    }

    pub fn neighbors(self, direction: Direction) -> impl Iterator<Item = Self> {
        struct PositionIterator<const LIMIT_X: usize, const LIMIT_Y: usize> {
            last: Position<LIMIT_X, LIMIT_Y>,
            direction: Direction,
        }

        impl<const LIMIT_X: usize, const LIMIT_Y: usize> Iterator for PositionIterator<LIMIT_X, LIMIT_Y> {
            type Item = Position<LIMIT_X, LIMIT_Y>;

            fn next(&mut self) -> Option<Self::Item> {
                let next = self.last.neighbor(self.direction)?;
                self.last = next;
                Some(next)
            }
        }

        PositionIterator {
            last: self,
            direction,
        }
    }

    pub fn value(self) -> u64 {
        let x = u64::from(self.x) + 1;
        let y = u64::from(self.y) + 1;
        x + 100 * y
    }
}

impl<const LIMIT_X: usize, const LIMIT_Y: usize> std::fmt::Display for Position<LIMIT_X, LIMIT_Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<I: TryInto<u8>, const LIMIT_X: usize, const LIMIT_Y: usize> TryFrom<(I, I)>
    for Position<LIMIT_X, LIMIT_Y>
{
    type Error = ();

    fn try_from((x, y): (I, I)) -> Result<Self, Self::Error> {
        let x = x.try_into().map_err(|_| ())?;
        let y = y.try_into().map_err(|_| ())?;

        if (x as usize) >= LIMIT_X || (y as usize) >= LIMIT_Y {
            return Err(());
        }

        Ok(Position { x, y })
    }
}

struct Board<const WIDTH: usize, const HEIGHT: usize> {
    content: [[CellContent; WIDTH]; HEIGHT],
    robot: Position<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize> std::ops::Index<Position<WIDTH, HEIGHT>>
    for Board<WIDTH, HEIGHT>
{
    type Output = CellContent;

    fn index(&self, index: Position<WIDTH, HEIGHT>) -> &Self::Output {
        &self.content[index.y as usize][index.x as usize]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::ops::IndexMut<Position<WIDTH, HEIGHT>>
    for Board<WIDTH, HEIGHT>
{
    fn index_mut(&mut self, index: Position<WIDTH, HEIGHT>) -> &mut Self::Output {
        &mut self.content[index.y as usize][index.x as usize]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn parse(
        lines: &mut impl Iterator<Item = impl Iterator<Item = char>>,
    ) -> Result<Self, String> {
        // First line is discardable
        lines.next();

        let mut robot: Option<Position<WIDTH, HEIGHT>> = None;
        let mut content = [[CellContent::None; WIDTH]; HEIGHT];

        for line in content
            .iter_mut()
            .enumerate()
            .map(|row| lines.next().map(|line| (line, row)))
        {
            let Some((line, (y, row))) = line else {
                return Err(format!("Expected {HEIGHT} lines of content"));
            };

            let mut line = line.peekable();

            if line.next() != Some('#') {
                return Err("Expected all map lines to start with #".to_owned());
            }

            for cell in row
                .iter_mut()
                .enumerate()
                .map(|map_cell| line.next().map(|char| (char, map_cell)))
            {
                let Some((cell, (x, map_cell))) = cell else {
                    return Err(format!("Expected {WIDTH} cells in the map"));
                };

                *map_cell = match cell {
                    '.' => CellContent::None,
                    'O' => CellContent::Box,
                    '@' => {
                        if let Some(existing) = robot {
                            return Err(format!("Expected only one robot, but found one at {existing:?} and ({x}, {y})"));
                        }

                        robot = Some(Position::new(x as u8, y as u8));

                        CellContent::Robot
                    }
                    '#' => CellContent::Wall,
                    cell => return Err(format!("Found unexpected cell value {cell}")),
                }
            }
        }

        let Some(robot) = robot else {
            return Err("Did not find a robot in map".to_owned());
        };

        Ok(Self { content, robot })
    }

    pub fn move_robot(&mut self, direction: Direction) {
        if let Err(robots) = self.verify_robots() {
            panic!("Pre move sanity check failed with {robots:?}")
        }

        let start_robot_position = self.robot;
        // We look from the current robot position in direction until we find an open space
        let Some(direct_neighbor) = self.robot.neighbor(direction) else {
            return;
        };

        if self[direct_neighbor] == CellContent::None {
            self.robot = direct_neighbor;
            self[direct_neighbor] = CellContent::Robot;
            self[start_robot_position] = CellContent::None;
            return;
        }

        let Some(first_empty) = direct_neighbor
            .neighbors(direction)
            .find_map(|position| match self[position] {
                CellContent::None => Some(Some(position)),
                // If we encounter a wall, we have to stop the search since it can't be moved
                CellContent::Wall => Some(None),
                CellContent::Robot => unreachable!("Search should always happen from the robot, and therefore never encounter the robot again, started at {start_robot_position} and looking at {position}"),
                CellContent::Box => None
            })
            .flatten()
        else {
            return;
        };

        // Now we can mark first_empty as a box, move the robot into direct_neighbor, and clear the current robot position
        self[first_empty] = CellContent::Box;
        self[direct_neighbor] = CellContent::Robot;
        self[start_robot_position] = CellContent::None;
        self.robot = direct_neighbor;

        if let Err(robots) = self.verify_robots() {
            panic!("After move from {start_robot_position} expected only {direct_neighbor}, found multiple robots {robots:?}")
        }
    }

    fn verify_robots(&self) -> Result<(), Vec<Position<WIDTH, HEIGHT>>> {
        let mut first = None;
        let mut robots = None;

        for (row, y) in self.content.iter().zip(0..) {
            for (cell, x) in row.iter().copied().zip(0..) {
                if cell != CellContent::Robot {
                    continue;
                }

                let position = Position { x, y };

                match (first, robots.as_mut()) {
                    (None, None) => first = Some(position),
                    (Some(first_position), None) => {
                        first = None;
                        robots = Some(vec![first_position, position]);
                    }
                    (None, Some(positions)) => positions.push(position),
                    (Some(_), Some(_)) => unreachable!(),
                }
            }
        }

        if let Some(robots) = robots {
            Err(robots)
        } else {
            Ok(())
        }
    }

    pub fn box_sum(&self) -> u64 {
        self.content
            .iter()
            .zip(0..)
            .flat_map(|(row, y)| {
                row.iter()
                    .zip(0..)
                    .map(move |(cell, x)| (Position::<WIDTH, HEIGHT>::new(x, y), cell))
            })
            .filter_map(|(position, cell)| {
                if *cell == CellContent::Box {
                    Some(position)
                } else {
                    None
                }
            })
            .map(Position::value)
            .sum()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::fmt::Display for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.content {
            for cell in line {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Direction {
    pub fn parse(input: char) -> Option<Self> {
        Some(match input {
            '<' => Direction::Left,
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => return None,
        })
    }
}

#[test]
fn test_part_one_small() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    let mut input = input.lines().map(|line| line.chars());

    let mut board = Board::<6, 6>::parse(&mut input).unwrap();

    // We then discard the bottom wall and spacer
    input.next();
    input.next();

    for input in input.flatten().filter_map(Direction::parse) {
        board.move_robot(input);
    }

    let sum = board.box_sum();

    assert_eq!(sum, 2028);
}

#[test]
fn test_part_one_big() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    let mut input = input.lines().map(|line| line.chars());

    let mut board = Board::<8, 8>::parse(&mut input).unwrap();

    // We then discard the bottom wall and spacer
    input.next();
    input.next();

    for input in input.flatten().filter_map(Direction::parse) {
        board.move_robot(input);
        println!("After {input} board is now\n{board}")
    }

    let sum = board.box_sum();

    assert_eq!(sum, 10092);
}

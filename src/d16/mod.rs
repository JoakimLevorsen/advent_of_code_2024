use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellContent {
    None { lowest_cost: Option<NonZeroU32> },
    Wall,
    Start,
    End,
}

impl std::fmt::Display for CellContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellContent::None { .. } => ' ',
                CellContent::Wall => '#',
                CellContent::Start => 'S',
                CellContent::End => 'E',
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

    pub const fn turn_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub const fn turn_anti_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn neighbors_with_costs(
        self,
        direction: Direction,
    ) -> impl Iterator<Item = (NonZeroU32, Self, Direction)> {
        let one = NonZeroU32::new(1).unwrap();
        let straight = self
            .neighbor(direction)
            .map(|position| (one, position, direction));

        let thousand_and_one = NonZeroU32::new(1001).unwrap();
        let clockwise = self
            .neighbor(direction.turn_clockwise())
            .map(|position| (thousand_and_one, position, direction.turn_clockwise()));

        let anti_clockwise = self
            .neighbor(direction.turn_anti_clockwise())
            .map(|position| (thousand_and_one, position, direction.turn_anti_clockwise()));

        [straight, clockwise, anti_clockwise].into_iter().flatten()
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
    start: Position<WIDTH, HEIGHT>,
    end: Position<WIDTH, HEIGHT>,
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

        let mut start: Option<Position<WIDTH, HEIGHT>> = None;
        let mut end: Option<Position<WIDTH, HEIGHT>> = None;
        let mut content = [[CellContent::None { lowest_cost: None }; WIDTH]; HEIGHT];

        for line in content
            .iter_mut()
            .zip(0..)
            .map(|row| lines.next().map(|line| (line, row)))
        {
            let Some((line, (row, y))) = line else {
                return Err(format!("Expected {HEIGHT} lines of content"));
            };

            let mut line = line.peekable();

            if line.next() != Some('#') {
                return Err("Expected all map lines to start with #".to_owned());
            }

            for cell in row
                .iter_mut()
                .zip(0..)
                .map(|map_cell| line.next().map(|char| (char, map_cell)))
            {
                let Some((cell, (map_cell, x))) = cell else {
                    return Err(format!("Expected {WIDTH} cells in the map"));
                };

                *map_cell = match cell {
                    '.' => CellContent::None { lowest_cost: None },
                    '#' => CellContent::Wall,
                    'S' => {
                        let found = Position { x, y };
                        if let Some(existing) = start {
                            return Err(format!("Found two starts, {existing} and {found}"));
                        }

                        start = Some(found);
                        CellContent::Start
                    }
                    'E' => {
                        let found = Position { x, y };
                        if let Some(existing) = end {
                            return Err(format!("Found two ends, {existing} and {found}"));
                        }

                        end = Some(found);
                        CellContent::End
                    }
                    cell => return Err(format!("Found unexpected cell value {cell}")),
                }
            }
        }

        match (start, end) {
            (Some(start), Some(end)) => Ok(Board {
                content,
                start,
                end,
            }),
            (None, _) => Err("Failed to find start".to_owned()),
            (_, None) => Err("Failed to find end".to_owned()),
        }
    }

    pub fn find_cheapest_path(&mut self) -> u64 {
        struct PathCandidate<const WIDTH: usize, const HEIGHT: usize> {
            running_cost: NonZeroU32,
            position: Position<WIDTH, HEIGHT>,
            direction: Direction,
        }

        let mut cheapest = None;
        let mut to_check: Vec<_> = self
            .start
            .neighbors_with_costs(Direction::Right)
            .map(|(running_cost, position, direction)| PathCandidate {
                running_cost,
                position,
                direction,
            })
            .collect();

        while let Some(run) = to_check.pop() {
            for (added_cost, position, direction) in
                run.position.neighbors_with_costs(run.direction)
            {
                let running_cost = run
                    .running_cost
                    .checked_add(added_cost.into())
                    .expect("All values to fit in cost");

                if position == self.end {
                    continue;
                }

                // We check our current tile if we're on a more optimal path
                match &mut self[position] {
                    CellContent::None { lowest_cost } => {
                        if lowest_cost.is_some_and(|value| value < running_cost) {
                            // Then we've already found a better path to this tile, give up
                            continue;
                        }
                        *lowest_cost = Some(running_cost);
                    }
                    CellContent::Wall | CellContent::Start => continue,
                    CellContent::End => {
                        cheapest = Some(cheapest.unwrap_or(running_cost).min(running_cost));
                        continue;
                    }
                }

                to_check.push(PathCandidate {
                    running_cost,
                    position,
                    direction,
                });
            }
        }

        cheapest.expect("Expected to find at least one path")
    }
}

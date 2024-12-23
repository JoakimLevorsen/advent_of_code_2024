#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn find_neighbors(self, height: usize, width: usize) -> impl Iterator<Item = Position> {
        [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let x = self.x.checked_add_signed(dx)?;

                if x >= width {
                    return None;
                }

                let y = self.y.checked_add_signed(dy)?;

                if y >= height {
                    return None;
                }

                Some(Position { x, y })
            })
    }
}

fn count_paths(pos: Position, input: &[&[u8]]) -> u32 {
    let Position { x, y } = pos;
    let value = input[y][x];
    let height = input.len();
    let width = input[0].len();

    pos.find_neighbors(height, width)
        // .filter(|neighbor| input[neighbor.y][neighbor.x] == value + 1)
        .map(|neighbor| match (value, input[neighbor.y][neighbor.x]) {
            (b'8', b'9') => 1,
            (current, next) if current + 1 == next => count_paths(neighbor, input),
            _ => 0,
        })
        .sum()
}

fn count_trails(input: &[&[u8]]) -> u32 {
    let mut trails = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != b'0' {
                continue;
            }

            let score = count_paths(Position { x, y }, input);

            println!("Found ({x}, {y}) with {score}");
            trails += score;
        }
    }

    trails
}

#[test]
fn test_part_one() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let input_bytes: Vec<_> = input.lines().map(str::as_bytes).collect();

    let trails = count_trails(&input_bytes);

    assert_eq!(trails, 36);
}

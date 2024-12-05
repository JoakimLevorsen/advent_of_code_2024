const INPUT: &str = "./src/d4/input.txt";

const EXPECTED_CHAIN: &str = "XMAS";

type AsciiGrid<'a> = &'a [&'a [u8]];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

fn find_valid_chains(
    from: Position,
    height: usize,
    width: usize,
) -> impl Iterator<Item = impl IntoIterator<Item = Position>> {
    let dirs = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    dirs.into_iter().filter_map(move |(dx, dy)| {
        let mut out = [from; 4];

        for (vector, out) in (1..=3).zip(out.iter_mut().skip(1)) {
            let x = from.x.checked_add_signed(vector * dx)?;

            if x >= width {
                return None;
            }

            let y = from.y.checked_add_signed(vector * dy)?;

            if y >= height {
                return None;
            }

            *out = Position { x, y };
        }

        Some(out)
    })
}

#[test]
fn sanity_valid_chains() {
    for chain in find_valid_chains(Position { x: 2, y: 2 }, 10, 10) {
        println!("Chain {:?}", chain.into_iter().collect::<Vec<_>>());
    }
}

fn find_xmas_in_grid(input: AsciiGrid) -> usize {
    let mut xmases = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut chain = EXPECTED_CHAIN.as_bytes().iter().copied();
            if Some(*cell) != chain.next() {
                continue;
            }

            xmases += find_valid_chains(Position { x, y }, input.len(), input[0].len())
                .filter_map(|from| {
                    from.into_iter()
                        .skip(1)
                        .zip(chain.clone())
                        .all(|(position, expected)| {
                            input.get(position.y).and_then(|row| row.get(position.x))
                                == Some(&expected)
                        })
                        .then_some(())
                })
                .count();
        }
    }

    xmases
}

#[test]
fn test_part_one() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();

    let xmases = find_xmas_in_grid(&lines);

    assert_eq!(xmases, 18);
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let lines: Vec<_> = file.lines().map(str::as_bytes).collect();

    let xmases = find_xmas_in_grid(&lines);

    println!("Found {xmases} xmases");

    Ok(())
}

fn find_cross_lines(origin: Position, height: usize, width: usize) -> Option<[[Position; 3]; 2]> {
    if origin.x == 0 || origin.x >= width - 1 {
        return None;
    }
    if origin.y == 0 || origin.y >= height - 1 {
        return None;
    }

    let diagonal = [[(1, 1), (0, 0), (-1, -1)], [(1, -1), (0, 0), (-1, 1)]];

    Some(diagonal.map(move |row| {
        row.map(move |(dx, dy)| Position {
            x: origin.x.checked_add_signed(dy).unwrap(),
            y: origin.y.checked_add_signed(dx).unwrap(),
        })
    }))
}

fn find_x_mas_in_grid(input: AsciiGrid) -> usize {
    let mut crosses = 0;

    for (y, row) in input.iter().enumerate() {
        'cell_loop: for (x, cell) in row.iter().enumerate() {
            if *cell != b'A' {
                continue;
            }

            let Some(set) = find_cross_lines(Position { x, y }, input.len(), row.len()) else {
                continue;
            };

            let found_values = set.map(|row| row.map(|position| input[position.y][position.x]));

            for row in found_values {
                if row != [b'M', b'A', b'S'] && row != [b'S', b'A', b'M'] {
                    continue 'cell_loop;
                }
            }

            crosses += 1;
        }
    }

    crosses
}

#[test]
fn test_part_two() {
    let file = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let lines: Vec<_> = file.lines().map(str::as_bytes).collect();

    let xmases = find_x_mas_in_grid(&lines);

    assert_eq!(xmases, 9);
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let lines: Vec<_> = file.lines().map(str::as_bytes).collect();

    let xmases = find_x_mas_in_grid(&lines);

    println!("Found {xmases} xmases");

    Ok(())
}

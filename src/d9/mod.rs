#[derive(PartialEq, Eq, Clone, Copy)]
enum Block {
    Empty,
    Filled { id: u16 },
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "NaN"),
            Self::Filled { id } => f.write_fmt(format_args!("{id}")),
        }
    }
}

impl Block {
    pub const fn is_empty(&self) -> bool {
        match self {
            Block::Empty => true,
            Block::Filled { .. } => false,
        }
    }
}

type Disk = Vec<Block>;

fn parse_disk(description: &str) -> Disk {
    let mut disk = vec![];

    let mut blocks = description
        .chars()
        .map(|char| char.to_digit(10).expect("All input should be decimal"));

    let mut next_id = 0;

    loop {
        // File
        let Some(length) = blocks.next() else {
            break;
        };
        for _ in 0..length {
            disk.push(Block::Filled { id: next_id });
        }
        next_id += 1;

        // Empty
        let Some(length) = blocks.next() else {
            break;
        };
        for _ in 0..length {
            disk.push(Block::Empty);
        }
    }

    disk
}

fn compress_disk(disk: &mut Disk) {
    let mut last_open = None;
    let mut last_moved = None;

    let mut holes = disk
        .iter()
        .rev()
        .skip_while(|block| Block::is_empty(block))
        .filter(|block| block.is_empty())
        .count();
    while holes > 0 {
        let Some(next_open) = disk
            .iter()
            .enumerate()
            .skip(last_open.unwrap_or(0))
            .find_map(|(id, block)| if block.is_empty() { Some(id) } else { None })
        else {
            return;
        };

        let Some(to_move) = disk
            .iter()
            .enumerate()
            .rev()
            .skip(disk.len() - last_moved.unwrap_or(disk.len()))
            .find_map(|(index, block)| if block.is_empty() { None } else { Some(index) })
        else {
            return;
        };

        disk.swap(next_open, to_move);
        holes -= 1;
        last_open = Some(next_open);
        last_moved = Some(to_move);
    }
}

fn find_checksum(disk: &Disk) -> u64 {
    disk.iter()
        .take_while(|block| !block.is_empty())
        .zip(0..)
        .map(|(block, index)| match block {
            Block::Empty => 0,
            Block::Filled { id } => u64::from(*id) * index,
        })
        .sum()
}

#[test]
fn test_part_one() {
    let input = "2333133121414131402";

    let mut disk = parse_disk(input);

    // println!("Found disk {disk:?}");

    compress_disk(&mut disk);

    println!("Disk {disk:?}");

    let expected_compressed = "0099811188827773336446555566..............";
    for (block, expected) in disk.iter().zip(expected_compressed.chars()) {
        match (block, expected) {
            (Block::Empty, '.') => continue,
            (Block::Filled { id }, '.') => panic!("Expected empty space, found block with id {id}"),
            (Block::Filled { id }, expected_id) => {
                assert_eq!(u32::from(*id), expected_id.to_digit(10).unwrap());
            }
            (Block::Empty, char) => panic!("Expected id {char} but found empty"),
        }
    }

    // println!("\nDisk is now {disk:?}");

    let checksum = find_checksum(&disk);

    assert_eq!(checksum, 1928);
}

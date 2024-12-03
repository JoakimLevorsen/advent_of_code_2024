const INPUT: &str = "./src/d2/input.txt";

fn validate_line<const TOLERANCE: u8>(line: &str) -> Result<(), ()> {
    let levels = line
        .split(' ')
        .map(str::parse::<u8>)
        .map(|result| result.map_err(|_| ()));

    let mut level_pairs = levels.clone().zip(levels.skip(1)).peekable();

    let (first, second) = level_pairs
        .peek()
        .copied()
        .expect("There should be at least two numbers in a line");

    // Need to either increase or decrease
    if first == second {
        return Err(());
    }
    let first_order = first?.cmp(&second?);

    let mut faults = 0;

    for (value, next) in level_pairs {
        let value = value?;
        let next = next?;
        if first_order != value.cmp(&next) {
            // Unsafe order
            faults += 1;
            if faults > TOLERANCE {
                return Err(());
            }
            continue;
        }

        if let 1..=3 = value.abs_diff(next) {
            // Safe
            continue;
        }

        faults += 1;
        if faults > TOLERANCE {
            return Err(());
        }
    }

    Ok(())
}

fn validate_lines_with_tolerance(line: &str) -> Result<(), ()> {
    let mut values = line
        .split(' ')
        .map(str::parse::<u8>)
        .map(|result| result.map_err(|_| ()))
        .peekable();

    let mut previous = values
        .next()
        .expect("There should be at least two values in the line")?;
    // let _next = values.peek().copied().unwrap().unwrap();
    let mut direction = values
        .peek()
        .copied()
        .expect("There should be at least two values in a line")?
        .cmp(&previous);

    let mut values = values.enumerate().peekable();

    let mut has_seen_faults = false;
    let mut consumed = 1;

    while let Some((index, next)) = values.next() {
        let next = next?;

        // Distance must be 1 to 3 otherwise we can drop the value immediately
        match next.abs_diff(previous) {
            1..=3 => (),
            _ if has_seen_faults == false => {
                has_seen_faults = true;
                continue;
            }
            _ => return Err(()),
        }

        if next.cmp(&previous) != direction {
            if has_seen_faults {
                // We only tolerate one error
                return Err(());
            }
            has_seen_faults = true;
            // If we're on the third value, we could still recover by dropping either the second or third value
            if index == 2 {
                let Some((_, Ok(fourth))) = values.peek().copied() else {
                    // We should be okay if we only have 3 values and we can just drop the third
                    return Ok(());
                };

                if direction == fourth.cmp(&previous) {
                    // In this case we drop the third value
                    continue;
                }
                if direction != fourth.cmp(&next) {
                    // Then we retroactively drop the second value
                    previous = next;
                    direction = fourth.cmp(&next);
                    continue;
                }
            }
            // Otherwise we just drop this value
            continue;
        }

        previous = next;
    }

    Ok(())
}

#[test]
fn validate_validate() {
    assert_eq!(validate_line::<0>("11 15 16 18 20 21 23 26"), Err(()));
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let mut safe = 0;
    for line in file.lines() {
        match validate_line::<0>(line) {
            Ok(()) => safe += 1,
            _ => continue,
        }
    }

    println!("Found {safe} reports");

    Ok(())
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let mut safe = 0;
    for line in file.lines() {
        match validate_lines_with_tolerance(line) {
            Ok(()) => safe += 1,
            _ => continue,
        }
    }

    println!("Found {safe} reports");

    Ok(())
}

#[test]
fn test_tolorence() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;
    let mut safe_counts = Vec::with_capacity(10);

    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<0>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<1>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<2>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<3>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<4>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<5>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<6>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }
    {
        let mut safe = 0;
        for line in file.lines() {
            match validate_line::<7>(line) {
                Ok(()) => safe += 1,
                _ => continue,
            }
        }

        safe_counts.push(safe);
    }

    println!("Found counts {safe_counts:?}");

    Ok(())
}

#[test]
fn part_two_test_set() {
    let file = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let mut safe = 0;
    let mut new_safe = 0;
    for line in file.lines() {
        match validate_line::<1>(line) {
            Ok(()) => safe += 1,
            _ => continue,
        }
        match validate_lines_with_tolerance(line) {
            Ok(()) => new_safe += 1,
            _ => continue,
        }
    }

    println!("{safe} {new_safe}");

    assert_eq!(safe, 4);
}

use std::iter::Peekable;

const INPUT: &str = "./src/d3/input.txt";

fn parse_number(iter: &mut Peekable<impl Iterator<Item = char>>) -> Option<u32> {
    let mut number = iter.peek().and_then(|char| char.to_digit(10))?;
    iter.next();

    let Some(second_digit) = iter.peek().and_then(|char| char.to_digit(10)) else {
        return Some(number);
    };
    iter.next();
    number = number * 10 + second_digit;

    let Some(third_digit) = iter.peek().and_then(|char| char.to_digit(10)) else {
        return Some(number);
    };
    iter.next();

    Some(number * 10 + third_digit)
}

fn parse_next_mul(iter: &mut Peekable<impl Iterator<Item = char>>) -> Option<(u32, u32)> {
    'outer_loop: loop {
        for char in "mul(".chars() {
            if iter.peek().copied()? != char {
                return None;
            }

            iter.next();
        }

        let Some(first) = parse_number(iter) else {
            continue 'outer_loop;
        };

        if iter.peek().copied()? != ',' {
            continue 'outer_loop;
        }
        iter.next();

        let Some(second) = parse_number(iter) else {
            continue 'outer_loop;
        };

        if iter.peek().copied()? != ')' {
            continue 'outer_loop;
        }
        iter.next();

        return Some((first, second));
    }
}

enum Condition {
    Do,
    Dont,
}

/// Will try to parse next condition, note does not consume any characters if first char does not match
fn parse_next_condition(iter: &mut Peekable<impl Iterator<Item = char>>) -> Option<Condition> {
    for char in "do".chars() {
        if iter.peek().copied()? != char {
            return None;
        }

        iter.next();
    }

    match iter.peek() {
        Some('n') => (),
        None | Some(_) => return Some(Condition::Do),
    }

    for char in "n't".chars() {
        if iter.peek().copied()? != char {
            return None;
        }

        iter.next();
    }

    Some(Condition::Dont)
}

fn parse_all_muls(input: &str) -> u32 {
    let mut chars = input.chars().peekable();
    let mut sum = 0;

    while chars.peek().is_some() {
        if let Some((a, b)) = parse_next_mul(&mut chars) {
            sum += a * b
        } else {
            chars.next();
        }
    }

    sum
}

fn parse_conditional_muls(input: &str) -> u32 {
    let mut chars = input.chars().peekable();
    let mut sum = 0;
    let mut enabled = true;

    while chars.peek().is_some() {
        match parse_next_condition(&mut chars) {
            Some(Condition::Do) => {
                enabled = true;
            }
            Some(Condition::Dont) => {
                enabled = false;
            }
            None if enabled == false => {
                // In this case we know a mul would be invalid, so we need to swallow the next iter value
                chars.next();
            }
            None => (),
        }

        if enabled == false {
            continue;
        }

        if let Some((a, b)) = parse_next_mul(&mut chars) {
            sum += a * b;
        } else {
            chars.next();
        }
    }

    sum
}

#[test]
fn test_part_one() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let sum = parse_all_muls(input);

    assert_eq!(sum, 161);
}

#[test]
fn test_part_of_input() {
    let mut  input = "%why();how()*-],+!mul(696,865)why()from()how():,;{where()mul(170,685)who()how()*from(881,957)?&select()mul(894,569):mul(648,114);[:from(657,891)how()mul(740,402)".chars().peekable();

    let pairs = [(696, 865), (170, 685), (894, 569), (648, 114), (740, 402)];

    for expected in pairs {
        let found = parse_next_mul(&mut input);

        assert_eq!(Some(expected), found);
    }
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let sum = parse_all_muls(&file);

    println!("Sum was {sum}");

    Ok(())
}

#[test]
fn test_part_two() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let sum = parse_conditional_muls(input);

    assert_eq!(sum, 48);
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let sum = parse_conditional_muls(&file);

    println!("Sum was {sum}");

    Ok(())
}

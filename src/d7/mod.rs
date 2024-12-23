const INPUT: &str = "./src/d7/input.txt";

use std::cmp::Ordering;

fn line_solvable(input: &str) -> Result<u128, ()> {
    let (result, inputs) = input.split_once(':').unwrap();
    let result = result.parse().unwrap();

    let inputs: Vec<u64> = inputs
        .trim_start()
        .split(' ')
        .map(|input| input.parse().expect("All lines should be valid numbers"))
        .collect();

    recursive_check(&result, 0, inputs.iter().copied()).map(|()| result.into())
}

fn recursive_check<I>(
    expected: &I,
    running_total: I,
    mut input: impl Iterator<Item = I> + Clone,
) -> Result<(), ()>
where
    I: Copy + std::ops::Add<I, Output = I> + std::ops::Mul<I, Output = I> + std::cmp::Ord,
{
    let Some(next_value) = input.next() else {
        if *expected == running_total {
            return Ok(());
        }
        return Err(());
    };
    match (running_total * next_value).cmp(expected) {
        Ordering::Less => {
            let deep = recursive_check(expected, running_total * next_value, input.clone());
            if deep.is_ok() {
                return Ok(());
            }
        }
        Ordering::Equal => {
            return if input.next().is_none() {
                Ok(())
            } else {
                Err(())
            }
        }
        Ordering::Greater => (),
    }

    match (running_total + next_value).cmp(expected) {
        Ordering::Less => recursive_check(expected, running_total + next_value, input),
        Ordering::Equal => {
            if input.next().is_none() {
                Ok(())
            } else {
                Err(())
            }
        }
        Ordering::Greater => Err(()),
    }
}

fn sum_of_solvable_lines(input: &str) -> u128 {
    input
        .lines()
        .map(line_solvable)
        .filter_map(std::result::Result::ok)
        .sum()
}

#[test]
fn test_part_one() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(sum_of_solvable_lines(input), 3749);
}

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let sum = sum_of_solvable_lines(&input);

    println!("Found sum {sum}");

    Ok(())
}

// 1297330561377 < too low
// 1297330561377

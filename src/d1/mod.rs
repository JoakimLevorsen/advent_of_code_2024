use std::collections::HashMap;

const INPUT: &str = "./src/d1/input.txt";

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let mut first = Vec::with_capacity(1_000);
    let mut second = Vec::with_capacity(1_000);

    for line in file.split('\n') {
        let Some((a, b)) = line.split_once("   ") else {
            break;
        };

        let a: u32 = a.parse()?;
        let b = b.parse()?;

        first.push(a);
        second.push(b);
    }

    first.sort_unstable();
    second.sort_unstable();

    let distance: u32 = first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("Total distance was: {distance}");

    Ok(())
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(INPUT)?;

    let mut first = Vec::with_capacity(1_000);
    let mut second = HashMap::with_capacity(500);

    for line in file.split('\n') {
        let Some((a, b)) = line.split_once("   ") else {
            break;
        };

        let a: u32 = a.parse()?;
        let b: u32 = b.parse()?;

        first.push(a);

        second.entry(b).and_modify(|count| *count += 1).or_insert(1);
    }

    let similarity: u32 = first
        .into_iter()
        .map(|number| number * second.get(&number).copied().unwrap_or(0))
        .sum();

    println!("Similarity was: {similarity}");

    Ok(())
}

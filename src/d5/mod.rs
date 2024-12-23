struct Entry {
    before: Vec<u8>,
    number: u8,
}

fn order_entries(mut entries: Vec<Entry>) -> Vec<u8> {
    let mut output = Vec::with_capacity(entries.len());

    let mut to_remove = vec![];
    let mut to_remove_inner = vec![];
    // We go through the list backwards and add the items to remove to the log
    while entries.is_empty() == false {
        for (index, entry) in entries.iter_mut().enumerate().rev() {
            for (index, value) in entry.before.iter().enumerate().rev() {
                if output.contains(value) {
                    to_remove_inner.push(index);
                }
            }

            for index in to_remove_inner.drain(..) {
                entry.before.swap_remove(index);
            }

            if entry.before.is_empty() == false {
                continue;
            }

            to_remove.push(index);
        }

        for index in to_remove.drain(..) {
            let removed = entries.swap_remove(index);

            output.push(removed.number);
        }
    }

    output
}

fn construct_ordering_list(input: &str) -> Vec<Entry> {
    let mut entries = std::collections::HashMap::new();

    for line in input.lines() {
        let (before, after) = line.split_once('|').unwrap();

        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        entries
            .entry(after)
            .and_modify(|entry: &mut Entry| entry.before.push(before))
            .or_insert_with(|| Entry {
                number: after,
                before: vec![before],
            });
    }

    entries.into_values().collect()
}

#[test]
fn test_order_entries() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

    let list = construct_ordering_list(input);

    let list = order_entries(list);

    println!("{list:?}");
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Lines;
use std::fs;

fn create_group(lines: &mut Lines) -> Option<HashSet<char>> {
    let mut next = lines.next()?;
    let mut group: HashSet<char> = HashSet::new();

    while !next.is_empty() {
        for ch in next.chars() {
            group.insert(ch);
        }

        match lines.next() {
            Some(line) => next = line,
            None => {break;}
        }
    }

    if group.is_empty() {
        return None;
    }

    Some(group)
}

fn create_group_all_required(lines: &mut Lines) -> Option<(usize, HashMap<char, usize>)> {
    let mut next = lines.next()?;
    let mut group: HashMap<char, usize> = HashMap::new();

    let mut people_count = 0;
    while !next.is_empty() {
        people_count += 1;
        for ch in next.chars() {
            let counter = group.entry(ch).or_insert(0);
            *counter += 1;
        }

        match lines.next() {
            Some(line) => next = line,
            None => {break;}
        }
    }

    if group.is_empty() {
        return None;
    }

    Some((people_count, group))
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input file");
    let mut lines = file_string.lines();

    let mut groups = Vec::new();

    while let Some(group) = create_group(&mut lines) {
        groups.push(group);
    }
    
    let mut sum = 0;

    for group in groups {
        sum += group.len();
    }

    println!("Total sum = {}", sum);

    let mut lines = file_string.lines();
    let mut groups = Vec::new();

    while let Some(group) = create_group_all_required(&mut lines) {
        groups.push(group);
    }

    sum = 0;
    for (people_count, group) in groups {
        for (_, times_answered) in &group {
            if *times_answered == people_count {
                sum += 1;
            }
        }
    }

    println!("Total sum of all yes {}", sum);
}

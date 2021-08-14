use std::collections::HashMap;
use std::fs;

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let lines = file_string.lines();
    let mut values: Vec<_> = lines.map(|s| s.parse::<usize>().expect("Error parsing values")).collect();

    let device_value = values.iter().max().unwrap() + 3;

    values.sort();

    let mut diffs: HashMap<usize,usize> = HashMap::new();

    // Outlet to first adapter
    *diffs.entry(values[0]).or_insert(0) += 1;

    for i in 0..values.len() - 1 {
        let diff = values[i + 1] - values[i];
        *diffs.entry(diff).or_insert(0) += 1;
    }

    let last_diff = device_value - *values.last().unwrap();
    // Last adapter to device
    *diffs.entry(last_diff).or_insert(0) += 1;

    let a = diffs[&1];
    let b = diffs[&3];
    let result = a * b;

    println!("{} * {} = {}", a, b, result);
}

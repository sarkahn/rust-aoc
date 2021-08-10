use std::fs;

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let lines = file_string.lines();
    let values: Vec<_> = lines.map(|s| s.parse::<usize>().expect("Error parsing values")).collect();

    let device_value = values.iter().max().unwrap() + 3;

    println!("Device value {}", device_value);
}

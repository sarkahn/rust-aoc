use std::fs;

fn get_values(line: &str) -> Vec<i32> {
    line.split(',').filter(
    |p| {
        p.chars().nth(0).unwrap() != 'x'
    }).map(
    |s| {
            s.parse::<i32>().expect("Error parsing values")
    }).collect()
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let lines: Vec<_> = file_string.lines().collect();

    let target = lines[0].parse::<i32>().expect("Error parsing departure time");
    
    let values = get_values(lines[1]);

    let mut min = i32::MAX;
    let mut id = 0;
    for value in values {
        let nearest = target as f32 / value as f32;
        let nearest = f32::ceil(nearest);
        let nearest = (value * nearest as i32) - target;
        if nearest < min {
            min = nearest;
            id = value;
        }
    }

    let result = id * min;

    println!("ID {}, wait time {}, Result {}", id, min, result);
}

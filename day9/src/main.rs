use std::fs;

fn check(range: &[usize], value: usize) -> Result<(usize,usize),String> {
    for i in 0..range.len() - 1 {
        for j in i + 1 ..range.len() {
            let i = range[i];
            let j = range[j];
            if i + j == value {
                return Ok((i,j));
            }
        }
    }
    Err(format!("No two values from the given range add up to {}", value))
}

fn get_matching_range(range: &[usize], target: usize) -> Result<&[usize],&str> {
    let mut val = 0;
    for i in 0..range.len() {
        val += range[i];
        if val == target {
            return Ok(&range[0..i]);
        } else if val > target {
            break;
        }
    }
    Err("No contiguous range found matching target value")
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");

    let lines = file_string.lines();

    let input: Vec<_> = lines.map(|s| s.parse::<usize>().expect("Error parsing lines")).collect();

    let mut invalid_number = 0;

    for i in 25..input.len() {
        let preamble = &input[i - 25..i];
        let value = input[i];

        match check(&preamble, value) {
            Ok((j,q)) => {
                //println!("Index {}: {} and {} add up to {}", i, j, q, value);
            },
            Err(_) => {
                println!("{} at index {} is invalid - no two values from the previous 25 add up to it", input[i], i);
                invalid_number = input[i];
                break;
            }
        }
    }

    for i in 0..input.len() {
        let range = &input[i..input.len()];
        match get_matching_range(range, invalid_number) {
            Ok(range) => {
                if range.len() < 1 {
                    continue;
                }
                let mut range = range.to_vec();
                range.sort();
                let weakness = range.first().unwrap() + range.last().unwrap();
                println!("Weakness found in range {:?}", range);
                println!("{}", weakness);
            },
            Err(_) => ()
        }
    }
}

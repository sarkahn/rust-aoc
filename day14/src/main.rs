use std::str::Lines;
use std::fs;
use std::collections::HashMap;

fn to_binary(num: u64) -> [i32; 36] {
    let mut arr = [0; 36];

    let mut n = num;
    let mut i = 35;
    while n > 0 {
        arr[i] = (n % 2) as i32;
        n = n / 2;
        i -= 1;
    }

    arr
}

fn to_digit(arr: [i32; 36]) -> u64 {
    let mut decimal = 0;
    let mut i = 35;
    let mut base: u64 = 1;
    while i > 0 {
        decimal += arr[i] as u64 * base;
        base *= 2;
        i -= 1;
    }
    decimal
}

// fn string_to_binary(string: &str) -> [i32; 36] {
//     let mut arr = [0; 36];
//     let mut nums: Vec<i32> = string.chars().map(|c| {
//         match c {
//             '1' => 1,
//             _ => 0
//         }
//     }).collect();

//     let mut i = 35;
//     for v in nums.iter().rev() {
//         arr[i] = *v;
//         i -= 1;
//     }
//     arr
// }

fn apply_mask(mask: &Vec<char>, bin: [i32; 36]) -> [i32; 36] {
    let mut bin = bin;
    for i in (0..36).rev() {
        match mask[i] {
            '1' => bin[i] = 1,
            '0' => bin[i] = 0,
            _ => {}
        }
    }
    
    bin
}

fn line_to_mask(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn process_writes(lines: &mut Lines, mem: &mut HashMap<i32,u64>) {
    let mut mask: Vec<char> = Vec::new();
    while let Some(line) = lines.next() {
        let split: Vec<_> = line.split(" = ").collect();
        match split[0] {
            "mask" => {
                mask = line_to_mask(split[1]);
                println!("Mask {:?}", mask);
            },
            _ => {
                let val = split[1].parse::<u64>().expect("Error parsing mem write value");
                let old_val = val;
                let bin = to_binary(val);
                let bin = apply_mask(&mask, bin);
                let val = to_digit(bin);

                let address: Vec<_> = split[0].split(|s| s == '[' || s == ']').collect();
                let address = address[1].parse::<i32>().expect("Error parsing mem address");

                println!("Processing val {}. NewVal {}. Applying to address {}", old_val, val, address);
                *mem.entry(address).or_insert(0) = val;

            }
        }
    }

    let mut sum = 0;
    for (_,value) in mem {
        sum = sum + *value;
    }

    println!("Sum: {}", sum);
}


fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let mut lines = file_string.lines();
    let mut mem = HashMap::new();


    // let bin = to_binary(0);
    // let mask = line_to_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    // let bin = apply_mask(&mask, bin);
    // let dig = to_digit(bin);

    process_writes(&mut lines, &mut mem);

    //println!("{:?}", bin);
    //println!("{:?}, {}", bin, dig);
}

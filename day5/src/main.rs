use std::fs;

fn split_sequence(sequence: &str, mut low: i32, mut high: i32) -> i32 {
    for ch in sequence.chars() {
        split_value(ch, &mut low, &mut high);
    }
    low
}

fn split_value(split_type: char, low: &mut i32, high: &mut i32) {
    let half = (1 + (*high - *low)) / 2;

    if split_type == 'F' || split_type == 'L' {
        *high = ((*high + 1) - half) - 1;
    }

    if split_type == 'B' || split_type == 'R' {
        *low = ((*low + 1) + half) - 1;
    }
}

fn get_row_column(sequence: &str) -> (i32, i32) {
    let split = sequence.split_at(sequence.len() - 3);

    let row = split_sequence(split.0, 0, 127);
    let column = split_sequence(split.1, 0, 7);

    (row,column)
}

fn get_seat_id(row_column: (i32, i32)) -> i32 {
    row_column.0 * 8 + row_column.1
}

fn get_seat_id_from_sequence(sequence: &str) -> i32 {
    get_seat_id(get_row_column(sequence))
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input data");
    
    let mut seat_ids: Vec<i32> = Vec::new();

    for line in file_string.lines() {
        seat_ids.push(get_seat_id_from_sequence(line));
    }

    let max = seat_ids.iter().max().unwrap();
    println!("Highest seat id: {}", max);

    seat_ids.sort();

    for i in 0..seat_ids.len() - 1 {
        let curr = seat_ids[i];
        let next = seat_ids[i + 1];
        if curr + 1 != next {
            println!("Missing seat between {} and {}", curr, next);
        }
    }
}

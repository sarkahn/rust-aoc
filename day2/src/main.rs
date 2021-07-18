use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Couldn't parse line"))
        .collect()
}

fn validate_position(letter: char, pos: usize, string: &str) -> bool {
    let string:Vec<char> = string.chars().collect();
    if pos >= string.len() || string[pos] != letter {
        false
    } else {
        true
    }
}

fn main() {
    let lines = lines_from_file("data/passwords.txt");
    
    let mut valid_count = 0;

    // First policy - count occurences of the letter
    for line in lines.iter() {
        let split:Vec<&str> = line.split(|v| v == '-' || v == ':' || v == ' ').filter(|&v| !v.is_empty())
            //.map(|s| s.to_string().split_whitespace().collect())
            .collect();

        let min:usize = split[0].parse().expect("error parsing min value");
        let max:usize = split[1].parse().expect("error parsing max value");
        let letter:char = split[2].parse().expect("error parsing letter");
        let password:String = split[3].parse().expect("error parsing password");
        let count = password.matches(letter).count();

        if count >= min && count <= max {
            valid_count = valid_count + 1;
        }
    }

    println!("First rule (letter count): There are {} valid passwords out of {}", valid_count, lines.len());

    valid_count = 0;

    //let lines = &lines[0..5];

    // Second policy - check letter by position, EXACTLY ONE position must contain a letter
    for line in lines.iter() {
        let split:Vec<&str> = line.split(|v| v == '-' || v == ':' || v == ' ').filter(|&v| !v.is_empty()).collect();

        let first:usize = split[0].parse::<usize>().expect("error parsing min value") - 1;
        let second:usize = split[1].parse::<usize>().expect("error parsing max value") - 1;
        let letter:char = split[2].parse().expect("error parsing letter");
        let password:String = split[3].parse().expect("error parsing password");

        let first_valid = validate_position(letter, first, &password);
        let second_valid = validate_position(letter, second, &password);

        if (first_valid == false && second_valid == false) || (first_valid == true && second_valid == true) {
            //println!("Positions {}, {}. Letter {}. Password {}. Is valid: {}", first, second, letter, password, true);
        } else {
            valid_count = valid_count + 1;
            //println!("Positions {}, {}. Letter {}. Password {}. Is valid: {}", first, second, letter, password, false );
        }
    }

    println!("Second rule (letter position): There are {} valid passwords out of {}", valid_count, lines.len());

}

use std::fs;

fn get_heading_label(heading: i32) -> char {
    match heading {
        0 => return 'N',
        90 => return 'E',
        180 => return 'S',
        270 => return 'W',
        _ => { panic!("Invalid heading: {}", heading); }
    }
}

fn get_move_delta(action: char, heading: i32, value: i32) -> (i32,i32,i32) {
    let mut x = 0;
    let mut y = 0;
    let mut dheading = 0;
    
    match action {
        'N' => y += value,
        'S' => y -= value,
        'E' => x += value,
        'W' => x -= value,
        'F' => return get_move_delta( get_heading_label(heading), heading, value),
        'L' => dheading = -value,
        'R' => dheading = value, 
        _ => {}
    }

    (x,y,dheading)
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let directions: Vec<_> = file_string.lines().map(
        |s| {
            let pair = s.split_at(1);
            let action = pair.0.chars().nth(0).unwrap();
            let value = pair.1.parse::<i32>().expect("Error parsing value");
            return (action, value)
    }).collect();

    let mut heading = 90;
    let mut x = 0;
    let mut y = 0;

    for (action, value) in directions {
        let (dx, dy, dheading) = get_move_delta(action, heading, value);
        x += dx;
        y += dy;
        heading = (heading + dheading).rem_euclid(360);
        //println!("{}:{} - Applying delta ({}, {}, {}), new pos {}, {}, Heading {}", action, value, dx, dy, dheading, x, y, heading );
    }

    println!("Final pos {},{}, heading {}. Distance: {}", x, y, heading, x.abs() + y.abs());
}
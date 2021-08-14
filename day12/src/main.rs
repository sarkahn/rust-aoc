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

fn get_move_delta_waypoint(action: char, value: i32, waypoint: (i32,i32)) -> (i32,i32) {
    let (mut x,mut y) = waypoint;
    match action {
        'N' => y += value,
        'S' => y -= value,
        'E' => x += value,
        'W' => x -= value,
        'F' => {
            x *= value;
            y *= value;
        },
        'L' => {
            let (rx,ry) = rotate_point(waypoint, -(value as f32));
            x = rx;
            y = ry;
        },
        'R' => {
            let (rx,ry) = rotate_point(waypoint, (value as f32));
            x = rx;
            y = ry;
        },
        _ => {}
    }
    (x,y)
}

fn rotate_point(waypoint: (i32,i32), angle: f32) -> (i32,i32) {
    let (x,y) = waypoint;
    let angle = -angle.to_radians();

    let rx = x as f32 * angle.cos() - y as f32 * angle.sin();
    let ry = x as f32 * angle.sin() + y as f32 * angle.cos();

    (rx.round() as i32, ry.round() as i32)
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

    for (action, value) in &directions {
        let (dx, dy, dheading) = get_move_delta(*action, heading, *value);
        x += dx;
        y += dy;
        heading = (heading + dheading).rem_euclid(360);
        //println!("{}:{} - Applying delta ({}, {}, {}), new pos {}, {}, Heading {}", action, value, dx, dy, dheading, x, y, heading );
    }

    println!("Part 1 final pos {},{}, heading {}. Distance: {}", x, y, heading, x.abs() + y.abs());

     let mut x = 0;
     let mut y = 0;

     let mut waypoint = (10,1);

    for (action, value) in directions {
        match action {
            'F' => {
                //println!("{}{} Moving ship from {}, {}. {:?} * {}", action, value, x, y, waypoint, value);
                x += waypoint.0 * value;
                y += waypoint.1 * value;
            }
            _ => {
                let (dx,dy) = get_move_delta_waypoint(action, value, waypoint);
                waypoint.0 = dx;
                waypoint.1 = dy;
            }
        }
    }

    println!("Part 2 final pos {}, {}. Waypoint {:?}. Distance: {}", x, y, waypoint, x.abs() + y.abs());

}
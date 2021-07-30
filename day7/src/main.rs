use std::fs;
use std::collections::HashMap;

fn parse_line(line: &str) -> (String, Vec<(i32, String)>) {
    let split : Vec<_> = line.split(" bags contain").collect();

    let base_color = String::from(split[0]);
    let contents = split[1];
    let contents = contents.split(',').filter(|&s| !s.is_empty());

    let mut vec = Vec::new();

    for part in contents {
        if part == " no other bags." {
            continue;
        }
        //println!("{}", part);
        let split: Vec<_> = part.split(" bag").collect();
        let color = split[0].split_at(3).1;
        let amount = 0;
        let split = part.split_at(2).0.trim_start();
        
        let amount = split.parse::<i32>().unwrap();
        //let amount = amount.parse::<i32>().unwrap();
        println!("{:?}", (amount, color));
        vec.push((amount, String::from(color)));
    }

    (base_color, vec)
}

fn can_contain(map: &HashMap<String,Vec<(i32, String)>>, base_color: &str, target_color: &str) -> bool {
    for (_, content) in &map[base_color] {
        if content == target_color || can_contain(&map, content, target_color) {
            return true;
        }
    }
    false
}

fn count_containers(map: &HashMap<String,Vec<(i32,String)>>, color: &str) -> i32 {
    let mut count = 0;
    for (bag,_) in map {
        if can_contain(&map, bag, color) {
            count += 1;
        }
    }

    count
}

fn count_total(map: &HashMap<String,Vec<(i32,String)>>, base_color: &str) -> i32 {

    if !map.contains_key(base_color) {
        return 0;
    }
    let mut count = 0;

    let contained = &map[base_color];

    for (amount, col) in contained {
        count += amount + amount * count_total(&map, col);
    }
    
    count
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input file");

    let mut map = HashMap::new();
    for line in file_string.lines() {
        let entry = parse_line(line);
        map.insert(entry.0, entry.1);
    }

    let count = count_containers(&map, "shiny gold");

    println!("{} bags can contain a shiny gold bag.", count);

    let count = count_total(&map, "shiny gold");

    println!("{} inside gold bag", count);
}

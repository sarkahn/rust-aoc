use std::str::Lines;
use std::collections::HashMap;
use std::fs;

fn get_next_entry(lines: &mut Lines) -> Option<HashMap<String,String>> {
    let mut entry : HashMap<String,String> = HashMap::new();

    let mut next_line = lines.next();

    while next_line != None {

        let line = next_line.unwrap();
        if line.is_empty() {
            return Some(entry);
        }

        let tokens: Vec<_> = line.split(|x| (x == ':') || (x == ' ')).collect();

        let mut iter = tokens.into_iter();

        let mut next = iter.next();

        while next != None {
            let key = next.unwrap();
            // Empty line == end of entry
            if key.is_empty() {
                return Some(entry);
            }
    
            let value = iter.next().unwrap();

            entry.insert(String::from(key), String::from(value));
            
            next = iter.next();
        }

        next_line = lines.next();
    }

    if entry.is_empty() {
        return None;
    }

    Some(entry)
}

fn validate_year(name: &str, year: &str, min: i32, max: i32) -> Result<i32, String> {
    let year = year.parse::<i32>().unwrap();
    if year < min || year > max {
        return Err(format!("{} year '{}' out of acceptable range [{}-{}]", name, year, min, max));
    } 
    Ok(year)
}

fn validate_height(height: &str) -> Result<(i32,String), String> {
    let split = height.split_at(height.len() - 2);
    let tail = split.1;

    if tail.len() != 2 || (tail != "cm" && tail != "in") {
        return Err("Missing or incorrect height type - must be 'in' or 'cm'".to_string());
    }

    let value = split.0.parse::<i32>();
    if !value.is_ok() {
        return Err("Error parsing height value".to_string());
    } 

    let value = value.unwrap();

    let pair = (value, String::from(tail));

    if pair.1 == "cm" && (value < 150 || value > 193) {
        return Err(format!("Height value {:?} is out of acceptable range [150-193] cm", pair).to_string());
    }
    
    if pair.1 == "in" && (value < 59 || value > 76) {
        return Err(format!("Height value {:?} is out of acceptable range [59-76] in", pair).to_string());
    }

    Ok((value, String::from(tail)))
}

fn validate_hair_color(token: &str) -> Result<String,String> {
    let split = token.split_at(1);
    if split.0 != "#" {
        return Err(format!("Hair color {} - must begin with '#'", token));
    }

    if split.1.chars().count() != 6 {
        return Err(format!("Hair color value {} must be exactly 6 characters", token));
    }

    let chars = split.1.chars();
    for ch in chars {
        if !ch.is_ascii_hexdigit() {
            return Err(format!("Hair color value {} must consist of hexdigit characters (0-9, a-f)", token));
        }
    }

    Ok(String::from(token))
}

fn validate_eye_color(token: &str) -> Result<(), String> {
    if token == "amb" || token == "blu" || token == "brn" ||
    token == "gry" || token == "grn" || token == "hzl" || token == "oth" {
        return Ok(());
    }
    Err("Error parsing eye color, must match valid colors".to_string())
}

fn validate_passport_id(token: &str) -> Result<(), String> {
    if token.chars().count() != 9 {
        return Err(format!("Invalid passport id {} - must be exactly 9 digits", token));
    }
    let chars = token.chars();
    for ch in chars {
        if !ch.is_ascii_digit() {
            return Err(format!("Invalid passport id {} - must be all digits", token));
        }
    }
    Ok(())
}

fn contains_required_fields(entry: &HashMap<String,String>) -> bool {
    entry.contains_key("byr") &&
    entry.contains_key("iyr") &&
    entry.contains_key("eyr") &&
    entry.contains_key("hgt") &&
    entry.contains_key("hcl") &&
    entry.contains_key("ecl") &&
    entry.contains_key("pid")
}

fn is_valid_simple(entry: &HashMap<String,String>) -> Result<(), String> {
    if !contains_required_fields(&entry) {
        return Err("Doesn't contain required fields".to_string());
    }
    Ok(())
}

fn is_valid(entry: &HashMap<String,String>) -> Result<(), String> {
    is_valid_simple(&entry)?;
    validate_height(&entry["hgt"])?;
    validate_year("Birth", &entry["byr"], 1920, 2002)?;
    validate_year("Issue", &entry["iyr"], 2010, 2020)?;
    validate_year("Expiration", &entry["eyr"], 2020, 2030)?;
    validate_height(&entry["hgt"])?;
    validate_hair_color(&entry["hcl"])?;
    validate_eye_color(&entry["ecl"])?;
    validate_passport_id(&entry["pid"])?;
    Ok(())
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input data");

    let mut lines = file_string.lines();

    let mut entries : Vec<HashMap<String,String>> = Vec::new();

    while let Some(entry) = get_next_entry(&mut lines) {
        entries.push(entry);
    }

    let mut valid_count = 0;
    for entry in &entries {
        if is_valid_simple(&entry).is_ok() {
            valid_count = valid_count + 1;
        }
    }

    println!("{} valid entries based on 'simple' rules", valid_count);

    valid_count = 0;
    for entry in &entries {
        if is_valid(&entry).is_ok() {
            valid_count = valid_count + 1;
        }
    }

    println!("{} valid entries based on 'strict' rules", valid_count);

    // for entry in entries {
    //     match is_valid(&entry) {
    //         Ok(()) => println!("Entry is valid"),
    //         Err(e) => println!("Entry is invalid - {}", e), 
    //     }
    // }
}

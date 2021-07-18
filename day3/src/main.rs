use std::fs;

struct Map {
    width: usize,
    height: usize,
    chars: Vec<char>,
}

impl Map {
    fn new(string: &str) -> Map {
        let map = fs::read_to_string("data/map.txt").expect("Error reading map");
        let lines: Vec<&str> = map.lines().collect();
        
        let width = lines[0].replace('\r', "").replace('\n', "").len();
        let height = lines.len();
        let map = map.replace('\n', "").replace('\r', "");
        let chars = map.chars().collect();

        Map {
            width: width,
            height: height,
            chars: chars
        }
    }

    fn read_position(&self, x:usize, y:usize) -> char {
        self.chars[y * self.width  + x]
    }

    fn advance(&self, x: &mut usize, y: &mut usize) {
        *x = (*x + 3) % self.width;
        *y = *y + 1;
    }
}


fn read_position(x:usize, y:usize, width: usize, map: &Vec<char>) -> char {
    map[y * width  + x]
}

fn advance(x: &mut usize, y: &mut usize, width: usize) {
    *x = (*x + 3) % width;
    *y = *y + 1;
}

fn main() {
    let map = fs::read_to_string("data/map.txt").expect("Error reading map");
    let map = Map::new(&map);

    println!("Map size {}, {}", map.width, map.height);
    
    let mut x:usize = 0;
    let mut y:usize = 0;

    let mut tree_amount = 0;

    while y < map.height {

        if map.read_position(x, y) == '#' {
            tree_amount = tree_amount + 1;
        }

        map.advance(&mut x, &mut y);
    }

    println!("{} trees encountered", tree_amount);
}

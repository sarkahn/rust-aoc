use std::fs;

struct Map {
    width: usize,
    height: usize,
    chars: Vec<char>,
}

impl Map {
    fn new(string: &str) -> Map {
        let lines: Vec<&str> = string.lines().collect();
        
        let width = lines[0].replace('\r', "").replace('\n', "").len();
        let height = lines.len();
        let map = string.replace('\n', "").replace('\r', "");
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

    fn advance(&self, x: &mut usize, y: &mut usize, slope: (usize, usize)) {
        *x = (*x + slope.0) % self.width;
        *y = *y + slope.1;
    }
}

fn count_trees_along_slope(slope: (usize,usize), map: &Map) -> usize {
    let mut x:usize = 0;
    let mut y:usize = 0;

    let mut tree_amount = 0;

    while y < map.height {

        if map.read_position(x, y) == '#' {
            tree_amount = tree_amount + 1;
        }

        map.advance(&mut x, &mut y, slope);
    }

    tree_amount
}

fn main() {
    let map_string = fs::read_to_string("data/map.txt").expect("Error reading map");
    let map = Map::new(&map_string);

    println!("Map size {}, {}", map.width, map.height);

    let tree_amount = count_trees_along_slope((3,1), &map);

    println!("{} trees encountered", tree_amount);

    // Part 2
    let mut trees_product = 0;

    let slopes = [
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ];

    for slope in slopes {
        let tree_amount = count_trees_along_slope(slope, &map);

        if trees_product == 0 {
            trees_product = tree_amount;
        } else {
            trees_product = trees_product * tree_amount;
        }
    }

    println!("Combined product of slopes: {}", trees_product);
}

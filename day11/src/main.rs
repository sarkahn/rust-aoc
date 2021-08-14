use std::fs;

static ADJACENT: [(i32,i32); 8] =
[(-1, 0),
 (-1,-1),
 (-1, 1),
 ( 0, 1),
 ( 0,-1),
 ( 1, 0),
 ( 1, 1),
 ( 1,-1)];

struct Chars2D {
    width: usize,
    height: usize,
    chars: Vec<char>,
}



impl Chars2D {
    fn new(string: &str) -> Chars2D {
        let lines: Vec<_> = string.lines().collect();
        let string = string.replace('\n', "").replace('\r', "");
    
        let chars: Vec<_> = string.chars().collect();
        let width = lines[0].replace('\n', "").replace('\r', "").len();
        let height = lines.len();

        Chars2D {
            width: width,
            height: height,
            chars:chars
        }
    }

    fn read_seat(&self, x: usize, y: usize) -> char {
        self.chars[y * self.width  + x]
    }

    fn set_seat(&mut self, x: usize, y: usize, value: char) {
        self.chars[y * self.width + x] = value;
    }

    fn seat_count(&self) {
        
        let empty_seats = self.count_seats_of_type('L');
        let full_seats = self.count_seats_of_type('#');
        let floor = self.count_seats_of_type('.');
        println!("Empty: {}, Full: {}, Floor: {}", empty_seats, full_seats, floor);
    }

    fn try_get_adjacent(&self, x: usize, y: usize, adj:(i32, i32) ) -> Option<char> {
        let x = x as i32 + adj.0;
        let y = y as i32 + adj.1;

        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }

        Some(self.read_seat(x as usize,y as usize))
    }

    fn try_get_adjacent_los(&self, x: usize, y: usize, adj:(i32,i32)) -> Option<char> {
        let mut x = x as i32;
        let mut y = y as i32;

        loop {
            x += adj.0;
            y += adj.1;

            if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                return None;
            }

            let value = self.read_seat(x as usize, y as usize);

            if value != '.' {
                return Some(value);
            }
        }
    }

    fn count_adj(&self, x: usize, y: usize, value: char) -> usize {
        let mut count = 0;
        for adj in ADJACENT {
            match self.try_get_adjacent(x,y, adj) {
                Some(adj_seat) => { 
                    if adj_seat == value {
                        count += 1; 
                    }
                }
                _ => {}
            }
        }
        count
    }

    fn count_adj_los(&self, x: usize, y: usize, value: char) -> usize {
        let mut count = 0;
        for adj in ADJACENT {
            match self.try_get_adjacent_los(x, y, adj) {
                Some(adj_los) => {
                    if adj_los == value {
                        count += 1;
                    }
                }
                _ => {}
            }
        }
        count
    }

    fn print(&self) {
        for y in 0..self.height {
            let i = y * self.width;
            let row = &self.chars[i..i + self.width];
            println!("{:?}", row);
        }
    }

    fn evaluate_seat(&self, x: usize, y: usize, change_value: usize) -> Option<(usize,usize,char)> {
        let seat = self.read_seat(x, y);

        if seat == 'L' {
            if self.count_adj(x, y, '#') == 0 {
                //println!("Seat L at {},{} had no adjacent full seats", x, y);
                return Some((x,y,'#'));
            }
        }

        if seat == '#' {
            if self.count_adj(x, y, '#') >= change_value {
                //println!("Seat # at {}, {} had 4 or more adjacent empty seats", x, y);
                return Some((x,y, 'L'));
            }
        }

        None
    }

    fn evaluate_test(&self, x: usize, y: usize, change_value: usize, count_func: fn(&Chars2D, usize, usize, char) -> usize) -> Option<(usize,usize,char)> {
        let seat = self.read_seat(x, y);

        if seat == 'L' {
            if count_func(self, x, y, '#') == 0 {
                //println!("Seat L at {},{} had no adjacent full seats", x, y);
                return Some((x,y,'#'));
            }
        }

        if seat == '#' {
            if count_func(self, x, y, '#') >= change_value {
                //println!("Seat # at {}, {} had 4 or more adjacent empty seats", x, y);
                return Some((x,y, 'L'));
            }
        }

        None
    }

    fn evaluate_all(&mut self, change_value: usize, count_func: fn(&Chars2D, usize, usize, char) -> usize) -> usize {
        let mut deltas: Vec<(usize,usize,char)> = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                //match self.evaluate_seat(x,y, change_value) {
                match self.evaluate_test(x,y, change_value, count_func) {
                        Some(delta) => deltas.push(delta),
                    None => ()//{ println!("NO CHANGE AT {}, {}", x, y); }
                }
            }
        }
        let count = deltas.len();

        //println!("Evaluation had {} changed seats", count);

        for (x,y,seat) in deltas {
            //println!("{}, {}, {}", x, y, seat);
            self.set_seat(x, y, seat);
        }

        count
    }

    // fn evaluate_test(&self, x: usize, y: usize, change_value: usize) {
    //     match self.evaluate_seat(x, y, change_value) {
    //         Some(delta) => {
    //             println!("Evaluate returned a delta seat change for {}, {}: {:?}", x, y, delta);
    //         },
    //         None => {
    //             println!("Evaluate returned None for {}, {}", x, y);
    //         },
    //     }
    // }

    fn count_seats_of_type(&self, t: char) -> usize {
        let mut count = 0;
        for c in &self.chars {
            if *c == t {
                count += 1;
            }
        }
        count
    }

    fn evaluate_until_no_changes(&mut self, change_value: usize, count_func: fn(&Chars2D, usize, usize, char) -> usize) {

        let mut count = 0;
        while self.evaluate_all(change_value, count_func) > 0 {
            count += 1;
            //self.seat_count();
        }
        println!("Took {} iterations for no changes.", count);
        self.seat_count();
    }
}

fn main() {
    let file_string =  fs::read_to_string("data/input.txt").expect("Error reading input");
    let mut seats = Chars2D::new(&file_string);

    seats.evaluate_until_no_changes(4, Chars2D::count_adj);

    let mut seats = Chars2D::new(&file_string);
    seats.evaluate_until_no_changes(5, Chars2D::count_adj_los);
}
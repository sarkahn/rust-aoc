use std::fs;

#[derive(Default)]
struct State {
    acc: i32,
    instruction_index: usize,
    instruction_counter: Vec<i32>,
    instructions: Vec<String>,
}

impl State {
    fn new(instructions: Vec<String>) -> State {
        let len = instructions.len();
        State {
            instructions: instructions,
            instruction_counter: vec!(0; len),
            ..Default::default()
        }
    }

    fn modify_index(&mut self, value: i32) {
        let mut i = self.instruction_index as i32;
        let old_index = i;
        i = (i + value).rem_euclid(self.instructions.len() as i32);
        self.instruction_index = i as usize;
        println!("Changing instruction index from {} to {}", old_index, i);
    }
}

fn parse_line(state: &mut State) -> Result<(), String> {
    let line = &state.instructions[state.instruction_index];
    let split = line.split_at(4);
    let instruction = split.0.trim_end();
    let value = split.1.replace("+", "").parse::<i32>().expect("Error parsing instruction value");

    

    match instruction {
        "acc" => {
            if state.instruction_counter[state.instruction_index] > 0 {
                return Err(format!("Infinite loop encountered at index {}", state.instruction_index));
            }

            let old_acc = state.acc;
            state.acc += value;
            state.instruction_counter[state.instruction_index] += 1;
            state.instruction_index += 1;
            println!("Modifying acc from {} to {}. Index {}, counter at index: {}", 
            old_acc, state.acc, state.instruction_index, state.instruction_counter[state.instruction_index]);
        },
        "jmp" => {
            state.modify_index(value);
        }
        _ => state.instruction_index += 1,
    }

    Ok(())
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let lines: Vec<_> = file_string.lines().collect();
    let instructions: Vec<_> = lines.iter().map(|s| String::from(*s)).collect();

    let mut state = State::new(
        instructions
    );

    loop {
        let res = parse_line(&mut state);
        if !res.is_ok(){
            println!("{}. ACC: {}", res.err().unwrap(), state.acc);
            break;
        }
    }

    // parse_line(&mut state).unwrap();
    // parse_line(&mut state).unwrap();
    // parse_line(&mut state).unwrap();
}

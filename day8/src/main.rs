use core::num::ParseIntError;
use std::fs;

#[derive(Default)]
struct State {
    acc: i32,
    instruction_index: usize,
    instruction_counter: Vec<i32>,
    instructions: Vec<(String,i32)>,
}

impl State {
    fn new(instructions: Vec<(String,i32)>) -> State {
        let len = instructions.len();
        State {
            instructions: instructions,
            instruction_counter: vec!(0; len),
            ..Default::default()
        }
    }

    fn modify_index(&mut self, value: i32) {
        let mut i = self.instruction_index as i32;
        //let old_index = i;
        i = (i + value).rem_euclid(self.instructions.len() as i32);
        self.instruction_index = i as usize;
        //println!("jmp {}, Changing instruction index from {} to {}", value, old_index, i);
    }

    fn invert_instruction(&mut self, index: usize) {
        let (mut instruction,value) = self.instructions[index].clone();
        //let oldInstruction = instruction.clone();
        instruction = String::from(invert_instruction(&instruction));
        //if oldInstruction != instruction {
        //    println!("Changed {} to {} at line {}", oldInstruction, instruction, index + 1)
        //}
        self.instructions[index] = (instruction,value);
    }
}

fn invert_instruction(instruction: &str) -> &str {
    match instruction {
        "jmp" => return "nop",
        "nop" => return "jmp",
        _ => return instruction
    }
}


fn parse_line(line: &str) -> Result<(String,i32), ParseIntError> {
    let split = line.split_at(4);
    let instruction = String::from(split.0.trim_end());
    let value = split.1.replace("+", "").parse::<i32>()?;
    Ok((instruction,value))
}

fn process_instruction_value((instruction,value): (&str,i32), state: &mut State) -> Result<i32,String> {
    if state.instruction_counter[state.instruction_index] > 0 {
        return Err(format!("Infinite loop encountered at index {}", state.instruction_index));
    }
    
    match instruction {
        "acc" => {
            //let old_acc = state.acc;
            state.acc += value;
            state.instruction_counter[state.instruction_index] += 1;
            state.instruction_index += 1;
            //println!("Modifying acc from {} to {}. Index {}, counter at index: {}", 
            //old_acc, state.acc, state.instruction_index, state.instruction_counter[state.instruction_index]);
        },
        "jmp" => {
            state.modify_index(value);
        }
        _ => state.instruction_index += 1,
    }
    Ok(state.acc)
}

fn run_program(state: &mut State) -> Result<(), usize> {
    loop {

        let (instruction,value) = &state.instructions[state.instruction_index].clone();

        let res = process_instruction_value((instruction,*value), state);
        
        if !res.is_ok() {
            let i = state.instruction_index;
            //println!("{}", res.err().unwrap());
            return Err(i);
        }

        if state.instruction_index >= state.instructions.len() - 1 {
            return Ok(());
        }
    }
}

fn main() {
    let file_string = fs::read_to_string("data/input.txt").expect("Error reading input");
    let lines: Vec<_> = file_string.lines().collect();

    let instructions: Vec<_> = lines.iter().map(|s| parse_line(s).unwrap()).collect();

    let mut state = State::new(
        instructions.clone()
    );

    run_program(&mut state).ok();

    println!("ACC after repeated instruction {}", state.acc);

    for i in 0..instructions.len() {
        let mut state = State::new(
            instructions.clone()
        );

        state.invert_instruction(i);
    
        let res = run_program(&mut state);

        if res.is_ok() {
            println!("ACC after program is successful: {}", state.acc);
            break;
        }
    }
}

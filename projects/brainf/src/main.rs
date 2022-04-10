use std::env;
use std::fs;
use std::io;
fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let program = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("The program: {}", program);
    let mut tape = [0u8; 30000];

    // Where the processor is at in the program
    let mut program_counter = 0;
    // Where the current cell is in the tape
    let mut address_pointer = 0;

    let program_len = program.len();

    // tracks how many loops we are currently in
    // let mut loop_count = 0u8;
    let mut loop_start_indices = vec![];

    while program_counter < program_len {
        let cell_value = tape[address_pointer];

        match program.chars().nth(program_counter).unwrap() {
            /*
            Jump to closing bracket if the current cell is zero
             */
            '[' => {
                if cell_value == 0 {
                    let mut inside_loop_count = 0;
                    // move to matching ]
                    loop {
                        program_counter += 1;
                        let current_char = program.chars().nth(program_counter).unwrap();
                        if current_char == '[' {
                            inside_loop_count += 1;
                        } else if current_char == ']' {
                            if inside_loop_count == 0 {
                                break;
                            }
                            inside_loop_count -= 1;
                        }
                    }
                } else {
                    loop_start_indices.push(program_counter);
                }
            }
            ']' => {
                // pop back to start of loop
                if cell_value == 0 {
                    loop_start_indices.pop();
                } else {
                    program_counter = loop_start_indices.pop().unwrap() - 1;
                }
            }

            '+' => {
                tape[address_pointer] = cell_value + 1;
            }
            '-' => {
                tape[address_pointer] = cell_value - 1;
            }
            '>' => {
                address_pointer = address_pointer + 1;
            }
            '<' => {
                address_pointer = address_pointer - 1;
            }
            // print char at current point in the tape
            '.' => {
                print!("{}", cell_value as char)
            }
            // take input from user and store it in the tape
            ',' => {
                let mut user_input = String::new();
                println!("Please input a number");
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                let user_input = match user_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                tape[address_pointer] = user_input;
            }
            _ => (),
        }
        program_counter += 1;
    }
    // println!("Full tape: {:?}", tape);
}

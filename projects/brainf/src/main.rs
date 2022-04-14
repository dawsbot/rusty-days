use std::env;
use std::fs;
use std::io;
use std::num::Wrapping;

fn run(program: String) -> [Wrapping<u8>; 30000] {
    let mut tape = [Wrapping(0u8); 30000];
    // Where the processor is at in the program
    let mut program_counter = 0;
    // Where the current cell is in the tape
    let mut address_pointer = 0;

    let program_len = program.len();

    // stores the start index of each loop we're inside
    let mut loop_start_indices = vec![];

    let program_chars: Vec<char> = program.chars().collect();

    while program_counter < program_len {
        let cell_value = tape[address_pointer];

        match program_chars[program_counter] {
            /*
            Jump to closing bracket if the current cell is zero
             */
            '[' => {
                if cell_value == Wrapping(0) {
                    let mut inside_loop_count = 0;
                    // move to matching ]
                    loop {
                        program_counter += 1;
                        let current_char = program_chars[program_counter];
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
                if cell_value == Wrapping(0) {
                    loop_start_indices.pop();
                } else {
                    // pop back to start of loop
                    program_counter = loop_start_indices.pop().unwrap() - 1;
                }
            }

            '+' => {
                tape[address_pointer] = cell_value + Wrapping(1);
            }
            '-' => {
                tape[address_pointer] = cell_value - Wrapping(1);
            }
            '>' => {
                address_pointer = address_pointer + 1;
            }
            '<' => {
                address_pointer = address_pointer - 1;
            }
            // print char at current point in the tape
            '.' => {
                print!("{}", (cell_value.0 as char))
            }
            // take input from user and store it in the tape
            ',' => {
                let mut user_input = String::new();
                println!("Please input a number");
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                let user_input = match user_input.trim().parse() {
                    Ok(num) => Wrapping(num),
                    Err(_) => continue,
                };
                tape[address_pointer] = user_input;
            }
            _ => (),
        }
        program_counter += 1;
    }
    return tape;
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let program = fs::read_to_string(filename).expect("Something went wrong reading the file");
    program
}

fn main() {
    let program = read_file();
    run(program);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_plus() {
        let res = run(String::from("+"));
        assert_eq!(res[0], Wrapping(1));
        assert_eq!(res[1], Wrapping(0));
    }

    #[test]
    fn test_overflow_minus() {
        let res = run(String::from("-"));
        assert_eq!(res[0], Wrapping(255));
    }
}

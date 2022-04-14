use std::env;
use std::fs;
use std::num::Wrapping;

#[derive(Debug)]
enum Tokens {
    Decrement,
    Increment,
    ShiftLeft,
    ShiftRight,
    Print,
    TakeInput,
    StartLoop,
    EndLoop,
    NewLine,
}

// step 1 - removes non-necessary characters
fn lex(program: String) -> Vec<Tokens> {
    let mut lexed: Vec<Tokens> = Vec::new();
    for c in program.chars() {
        match c {
            '+' => lexed.push(Tokens::Increment),
            '-' => lexed.push(Tokens::Decrement),
            '<' => lexed.push(Tokens::ShiftLeft),
            '>' => lexed.push(Tokens::ShiftRight),
            '.' => lexed.push(Tokens::Print),
            ',' => lexed.push(Tokens::TakeInput),
            '[' => lexed.push(Tokens::StartLoop),
            ']' => lexed.push(Tokens::EndLoop),
            '\n' => lexed.push(Tokens::NewLine),
            _ => (),
        }
    }
    return lexed;
}

#[derive(Debug)]
struct ASTEntry {
    token: Tokens,
    line: usize,
}

// step 2
fn parse(lexed: Vec<Tokens>) -> Vec<ASTEntry> {
    let mut ast: Vec<ASTEntry> = Vec::new();
    let mut current_line = 1;
    for token in lexed {
        match token {
            Tokens::NewLine => current_line += 1,
            _ => {
                let ast_entry = ASTEntry {
                    token,
                    line: current_line,
                };
                ast.push(ast_entry);
            }
        }
    }
    return ast;
}

fn run(program: String) -> [Wrapping<u8>; 30000] {
    let mut tape = [Wrapping(0u8); 30000];
    // Where the processor is at in the program

    let mut program_counter = 0;
    // Where the current cell is in the tape
    let mut address_pointer = 0;

    let program_len = program.len();

    // stores the start index of each loop we're inside
    // let mut loop_start_indices = vec![];

    let lexed = lex(program);
    let parsed = parse(lexed);
    dbg!(parsed);

    // while program_counter < program_len {
    //     let cell_value = tape[address_pointer];

    //     match program_chars[program_counter] {
    //         /*
    //         Jump to closing bracket if the current cell is zero
    //          */
    //         '[' => {
    //             if cell_value == Wrapping(0) {
    //                 let mut inside_loop_count = 0;
    //                 // move to matching ]
    //                 loop {
    //                     program_counter += 1;
    //                     let current_char = program_chars[program_counter];
    //                     if current_char == '[' {
    //                         inside_loop_count += 1;
    //                     } else if current_char == ']' {
    //                         if inside_loop_count == 0 {
    //                             break;
    //                         }
    //                         inside_loop_count -= 1;
    //                     }
    //                 }
    //             } else {
    //                 loop_start_indices.push(program_counter);
    //             }
    //         }
    //         ']' => {
    //             if cell_value == Wrapping(0) {
    //                 loop_start_indices.pop();
    //             } else {
    //                 // pop back to start of loop
    //                 program_counter = loop_start_indices.pop().unwrap() - 1;
    //             }
    //         }

    //         '+' => {
    //             tape[address_pointer] = cell_value + Wrapping(1);
    //         }
    //         '-' => {
    //             tape[address_pointer] = cell_value - Wrapping(1);
    //         }
    //         '>' => {
    //             if address_pointer == 29999 {
    //                 address_pointer = 0;
    //             } else {
    //                 address_pointer = address_pointer + 1;
    //             }
    //         }
    //         '<' => {
    //             if address_pointer == 0 {
    //                 address_pointer = 29999;
    //             } else {
    //                 address_pointer = address_pointer - 1;
    //             }
    //         }
    //         // print char at current point in the tape
    //         '.' => {
    //             print!("{}", (cell_value.0 as char))
    //         }
    //         // take input from user and store it in the tape
    //         ',' => {
    //             let mut user_input = String::new();
    //             println!("Please input a number");
    //             io::stdin()
    //                 .read_line(&mut user_input)
    //                 .expect("Failed to read line");
    //             let user_input = match user_input.trim().parse() {
    //                 Ok(num) => Wrapping(num),
    //                 Err(_) => continue,
    //             };
    //             tape[address_pointer] = user_input;
    //         }
    //     }
    // program_counter += 1;
    // }
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
        let program = "+";
        let res = run(String::from(program));
        assert_eq!(res[0], Wrapping(1));
        assert_eq!(res[1], Wrapping(0));
    }

    #[test]
    fn test_overflow_minus() {
        let program = "-";
        let res = run(String::from(program));
        assert_eq!(res[0], Wrapping(255));
        assert_eq!(res[1], Wrapping(0));
    }
}

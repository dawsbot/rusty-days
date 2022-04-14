use std::env;
use std::fs;
use std::io;
use std::num::Wrapping;

#[derive(Debug, Clone)]
enum Tokens {
    Decrement,
    Increment,
    ShiftLeft,
    ShiftRight,
    Print,
    TakeInput,
    StartLoop,
    EndLoop,
    // NewLine,
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
            // '\n' => lexed.push(Tokens::NewLine),
            _ => (),
        }
    }
    return lexed;
}

#[derive(Debug, Clone)]
struct ASTEntry {
    token: Tokens,
    // line: usize,
    members: Vec<ASTEntry>,
}

// step 2
fn parse(lexed: Vec<Tokens>) -> Vec<ASTEntry> {
    let mut ast: Vec<ASTEntry> = vec![];
    let mut i = 0;
    while i < lexed.len() {
        let token = &lexed[i];
        let mut ast_entry = ASTEntry {
            token: token.clone(),
            members: Vec::new(),
        };
        match token {
            // newlines do not belong in the AST
            // Tokens::NewLine => current_line += 1,
            // Tokens::NewLine => continue,
            Tokens::StartLoop => {
                let nested_lexed = &lexed[(i + 1)..];
                let members = parse(nested_lexed.to_vec());
                ast_entry.members = members;
                // operations were inside this loop
                let members_len = ast_entry.members.len();
                if members_len > 0 {
                    i = i + members_len;
                }
                ast.push(ast_entry);
            }
            Tokens::EndLoop => {
                ast.push(ast_entry);
                return ast;
            }
            _ => {
                ast.push(ast_entry);
            }
        }
        i += 1;
    }
    return ast;
}

fn run(program: String) -> [Wrapping<u8>; 30000] {
    let lexed = lex(program);
    dbg!(&lexed);
    let parsed_program = parse(lexed);
    dbg!(&parsed_program);

    let tape = &mut [Wrapping(0u8); 30000];

    recurse_leaves(parsed_program, tape);
    fn recurse_leaves(ast: Vec<ASTEntry>, tape: &mut [Wrapping<u8>; 30000]) {
        let ast_len = ast.len();
        // Where the processor is at in the program
        let mut program_counter = 0;
        // Where the current cell is in the tape
        let mut address_pointer = 0;

        while program_counter < ast_len {
            // dbg!(program_counter, ast_len);
            let cell_value = tape[address_pointer];

            match ast[program_counter].token {
                /*
                Recurse in if non-zero
                  */
                Tokens::StartLoop => {
                    if cell_value != Wrapping(0) {
                        let members = ast[program_counter].members.clone();
                        // recurse into members
                        recurse_leaves(members, tape);
                        // let mut inside_loop_count = 0;
                        // // move to matching ]
                        // loop {
                        //     program_counter += 1;
                        //     let current_char = program_chars[program_counter];
                        //     if current_char == '[' {
                        //         inside_loop_count += 1;
                        //     } else if current_char == ']' {
                        //         if inside_loop_count == 0 {
                        //             break;
                        //         }
                        //         inside_loop_count -= 1;
                        //     }
                    }
                }
                Tokens::EndLoop => {
                    // println!("end loop, cell_value {}", cell_value);
                    if cell_value != Wrapping(0) {
                        recurse_leaves(ast.clone(), tape);
                    }
                }
                Tokens::Increment => {
                    // println!(
                    //     "incrementing cell_value: {}, curr: {}",
                    //     cell_value, tape[address_pointer]
                    // );
                    tape[address_pointer] = cell_value + Wrapping(1);
                }
                Tokens::Decrement => {
                    tape[address_pointer] = cell_value - Wrapping(1);
                }
                Tokens::ShiftRight => {
                    if address_pointer == 29999 {
                        address_pointer = 0;
                    } else {
                        address_pointer = address_pointer + 1;
                    }
                }
                Tokens::ShiftLeft => {
                    if address_pointer == 0 {
                        address_pointer = 29999;
                    } else {
                        address_pointer = address_pointer - 1;
                    }
                }
                // print char at current point in the tape
                Tokens::Print => {
                    print!("{}", (cell_value.0 as char))
                }
                // take input from user and store it in the tape
                Tokens::TakeInput => {
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
            }
            program_counter += 1;
        }
    }
    return *tape;
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
    #[test]
    fn test_small_loop() {
        let program = "+[-]";
        let res = run(String::from(program));
        assert_eq!(res[0], Wrapping(0));
    }
    #[test]
    fn test_comment_loop() {
        let program = "+>[+should skip this+]";
        let res = run(String::from(program));
        assert_eq!(res[0], Wrapping(1));
        assert_eq!(res[1], Wrapping(0));
    }
}

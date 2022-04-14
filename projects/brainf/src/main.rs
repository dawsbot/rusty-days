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
    let parsed_program = parse(lexed);

    let tape = &mut [Wrapping(0u8); 30000];
    recurse_leaves(parsed_program, tape, 0);

    fn recurse_leaves(ast: Vec<ASTEntry>, tape: &mut [Wrapping<u8>; 30000], mut tape_index: usize) {
        let ast_len = ast.len();
        // Where the processor is at in the program
        let mut ast_index = 0;

        // error address_pointer needs to be nested-aware to access value from the tape properly
        while ast_index < ast_len {
            let cell_value = tape[tape_index];
            println!();
            println!("{:?}", &tape[0..4]);
            dbg!(ast_index, tape_index, &ast[ast_index].token, cell_value);
            match ast[ast_index].token {
                /*
                Recurse in if non-zero
                  */
                Tokens::StartLoop => {
                    if cell_value != Wrapping(0) {
                        let members = ast[ast_index].members.clone();
                        dbg!(&members);
                        // recurse into members
                        recurse_leaves(members, tape, tape_index);
                    }
                }
                Tokens::EndLoop => {
                    // println!("end loop, cell_value {}", cell_value);
                    if cell_value != Wrapping(0) {
                        recurse_leaves(ast.clone(), tape, tape_index);
                    }
                }
                Tokens::Increment => {
                    tape[tape_index] = cell_value + Wrapping(1);
                }
                Tokens::Decrement => {
                    tape[tape_index] = cell_value - Wrapping(1);
                }
                Tokens::ShiftRight => {
                    if tape_index == 29999 {
                        tape_index = 0;
                    } else {
                        tape_index = tape_index + 1;
                    }
                }
                Tokens::ShiftLeft => {
                    if tape_index == 0 {
                        tape_index = 29999;
                    } else {
                        tape_index = tape_index - 1;
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
                    tape[tape_index] = user_input;
                }
            }
            ast_index += 1;
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
    #[test]
    fn test_nested_loop() {
        let program = "+>+[+>] should result in 120";
        let res = run(String::from(program));
        assert_eq!(res[0], Wrapping(1));
        assert_eq!(res[1], Wrapping(2));
        assert_eq!(res[2], Wrapping(0));
    }
}

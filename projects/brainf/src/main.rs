use std::env;
use std::fs;
use std::io;
use std::num::Wrapping;

#[derive(Debug, Clone, PartialEq)]
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
    consecutive_count: usize,
    // only opening and closing brackets get this
    matching_bracket: Option<usize>,
}

// step 2
// recursive-descent parser was turning out difficult, so made this an optimized interpreter instead
fn assemble_tokens(lexed: Vec<Tokens>) -> Vec<ASTEntry> {
    let mut tokens: Vec<ASTEntry> = vec![];
    let mut i = 0;

    // to know when we are inside a loop
    let mut opening_loop_indices: Vec<usize> = vec![];

    while i < lexed.len() {
        let token = &lexed[i];
        let mut ast_entry = ASTEntry {
            token: token.clone(),
            matching_bracket: None,
            consecutive_count: 1,
        };
        match token {
            Tokens::StartLoop => {
                tokens.push(ast_entry);
                opening_loop_indices.push(tokens.len() - 1);
            }
            Tokens::EndLoop => {
                let matching_bracket = opening_loop_indices.pop().unwrap();
                tokens[matching_bracket].matching_bracket = Some(tokens.len() - 1);
                ast_entry.matching_bracket = Some(matching_bracket);
                tokens.push(ast_entry);
            }
            _ => {
                let genesis_token = token;
                while i < lexed.len() - 1 && lexed[i + 1] == *genesis_token {
                    ast_entry.consecutive_count += 1;
                    i += 1;
                }
                tokens.push(ast_entry);
            }
        }
        i += 1;
    }
    // dbg!(&tokens);
    return tokens;
}

const MEMORY_SIZE: usize = 30_000;
fn run(program: String) -> [Wrapping<u8>; MEMORY_SIZE] {
    let lexed = lex(program);
    let tokens = assemble_tokens(lexed);

    let mut tape = [Wrapping(0u8); MEMORY_SIZE];
    let mut tape_index = 0;

    let mut tokens_index = 0;
    let tokens_len = tokens.len();

    // error address_pointer needs to be nested-aware to access value from the tape properly
    while tokens_index < tokens_len {
        let cell_value = tape[tape_index];
        let consecutive_count = tokens[tokens_index].consecutive_count;
        match tokens[tokens_index].token {
            Tokens::StartLoop => {
                if cell_value == Wrapping(0) {
                    tokens_index = tokens[tokens_index].matching_bracket.unwrap();
                }
            }
            Tokens::EndLoop => {
                // println!("end loop, cell_value {}", cell_value);
                if cell_value != Wrapping(0) {
                    tokens_index = tokens[tokens_index].matching_bracket.unwrap() - 1;
                }
            }
            Tokens::Increment => {
                tape[tape_index] = cell_value + Wrapping(consecutive_count as u8);
            }
            Tokens::Decrement => {
                tape[tape_index] = cell_value - Wrapping(consecutive_count as u8);
            }
            Tokens::ShiftRight => {
                tape_index = (tape_index + consecutive_count) % (MEMORY_SIZE - 1);
            }
            Tokens::ShiftLeft => {
                tape_index = ((MEMORY_SIZE - consecutive_count) + tape_index) % MEMORY_SIZE;
            }
            // print char at current point in the tape
            Tokens::Print => {
                for _ in 0..consecutive_count {
                    print!("{}", (cell_value.0 as char))
                }
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
        tokens_index += 1;
    }
    // dbg!(&tape);
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
        let program = "<-";
        let res = run(String::from(program));
        assert_eq!(res[MEMORY_SIZE - 1], Wrapping(255));
        assert_eq!(res[0], Wrapping(0));
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
        let program = "+>+[+>+[-<]]";
        let res = run(String::from(program));
        assert_eq!(res[0], Wrapping(0));
        assert_eq!(res[1], Wrapping(1));
        assert_eq!(res[2], Wrapping(0));
    }
}

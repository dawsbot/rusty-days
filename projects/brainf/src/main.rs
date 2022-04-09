use std::io;
fn main() {
    let program = ",.";
    // let program = ",.+>>+......";
    let mut tape = [0u8; 100];
    let mut pointer: usize = 0;

    for c in program.chars() {
        if c == '+' {
            tape[pointer] = tape[pointer] + 1;
        }
        if c == '-' {
            tape[pointer] = tape[pointer] - 1;
        }
        if c == '>' {
            pointer = pointer + 1;
        }
        if c == '<' {
            pointer = pointer - 1;
        }
        if c == '.' {
            println!("{}", tape[pointer] as char)
        }
        if c == ',' {
            let mut user_input = String::new();
            println!("Please input a number");
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            let user_input = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            tape[pointer] = user_input;
        }
    }

    // println!("Full tape: {:?}", tape);
}

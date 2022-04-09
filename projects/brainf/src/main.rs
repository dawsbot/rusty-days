use std::io;
fn main() {
    // let program = ",.";
    let program = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]
    <.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.";

    // let program = "++++[>+++++<-]>[<+++++>-]+<+[
    //     >[>+>+<<-]++>>[<<+>>-]>>>[-]++>[-]+
    //     >>>+[[-]++++++>>>]<<<[[<++++++++<++>>-]+<.<[>----<-]<]
    //     <<[>>>>>[>>>[-]+++++++++<[>-<-]+++++++++>[-[<->-]+[<<<]]<[>+<-]>]<<-]<<-
    // ]";
    // let program = ",.+>>+......";
    let mut tape = [0u8; 30000];

    // Where the processor is at in the program
    let mut program_counter = 0;
    // Where the current cell is in the tape
    let mut address_pointer = 0;

    let program_len = program.len();
    while program_counter < program_len {
        let current_char = program.chars().nth(program_counter).unwrap();

        // loops
        /*
        Jump to closing bracket if the current cell is zero
         */
        if current_char == '[' {
            if tape[address_pointer] == 0 {
                // find index of next ]
                while program.chars().nth(program_counter).unwrap() != ']' {
                    program_counter = program_counter + 1;
                }
            }
        }
        if current_char == ']' {
            if tape[address_pointer] != 0 {
                // find index of next ]
                while program.chars().nth(program_counter).unwrap() != '[' {
                    program_counter = program_counter - 1;
                }
            }
        }

        if current_char == '+' {
            tape[address_pointer] = tape[address_pointer] + 1;
        }
        if current_char == '-' {
            tape[address_pointer] = tape[address_pointer] - 1;
        }
        if current_char == '>' {
            address_pointer = address_pointer + 1;
        }
        if current_char == '<' {
            address_pointer = address_pointer - 1;
        }
        // print char at current point in the tape
        if current_char == '.' {
            print!("{}", tape[address_pointer] as char)
        }
        // take input from user and store it in the tape
        if current_char == ',' {
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
        program_counter += 1;
    }

    // println!("Full tape: {:?}", tape);
}

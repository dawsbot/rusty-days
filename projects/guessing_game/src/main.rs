use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    loop {
        // define this hear before storing user input here
        // mut is needed because all variables are immutable by default
        let mut guess = String::new();
        println!("Please input your guess");
        io::stdin()
            .read_line(&mut guess)
            /*
             * The right way to handle the error is to actually write error handling, but in our case we just want to crash this program when a problem occurs, so we can use expect
             */
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let answer = rand::thread_rng().gen_range(1..101);
        println!("You guessed: {}", guess);
        println!("answer: {}", answer);

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

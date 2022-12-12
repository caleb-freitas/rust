use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess.");
        
        // String::new() creates an empty instance of a `String`
        let mut guess = String::new();

        // The `::` syntax indicates that `stdin` is an associated function of the `io` function
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // shadow the previous value of `guess` with a new one as a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The `{}` syntax is a placeholder
        println!("You guessed: {guess}");

        // match expression
        match guess.cmp(&secret_number) {
            // each of this below expressions are called `arms`
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win. Congratulations!");
                break;
            }        
        }
    }
}


use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println! ("Guess the number!"); //prints to screen (macro)

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

        println! ("Please input your guess."); //prints to screen (macro)

        let mut guess = String::new(); //creates a mutable variable called guess. Equal sign binds it to a new empty instance of String.

        io::stdin() //allows us to handle the user input
            .read_line(&mut guess) //tells it what string to store the user input in. & is a reference and is immutable by default, so mut is added.
            .expect("Failed to read line"); //handles a potential failure result

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println! ("You guessed: {}", guess); //prints string with user's input in the {}placeholder

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

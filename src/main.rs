pub mod data_types;
pub mod variables;
pub mod functions;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Enter the guessing input !!!");

        let random_number = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();
        //"let" keyword is used to declare a variable
        //and "mut" keyword is used to make the variable as mutable
        // if mut keyword isn't there it is intended to work as immutable variable.
        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small !!"),
            Ordering::Greater => println!("Too Large !!"),
            Ordering::Equal => {
                println!("Bingoooo !!");
                break;
            }
        }
        println!("you guessed :{guess}");
        println!("The random number :{random_number}");

        variables::main();
        data_types::main();
    }
}

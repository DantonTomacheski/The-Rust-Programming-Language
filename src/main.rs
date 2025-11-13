use rand::{Rng};
use std::io;
use std::cmp::Ordering;
use colored::Colorize;

fn guess_the_number(){
    loop {
        // Generate a random number
        let mut rand = rand::rng();
        let random_number = rand.random_range(1..=100);

        // Ask the user a number and Read the terminal number
        let mut typed_number: String = String::new(); 
        println!("Can you type a number please?");
        io::stdin()
            .read_line(&mut typed_number)
            .expect("Error reading the number");
        
        // Normalize the number
        let typed_number: i32 = typed_number
            .trim()
            .parse()
            .expect("Error converting to a number");

        // Comparare the number to the random number
        match typed_number.cmp(&random_number) {
            Ordering::Greater => println!("The value is too much big, the real number is: {}", random_number),
            Ordering::Less => println!("The value is too small, the real number is: {}", random_number),
            Ordering::Equal => {
                println!("{}", "You got it! {}".blue());
                break;
            }
        }
    }
    
    // Return a bye bye
    println!("{}", "Bye bye buddy!!".green())
}

fn main() {
    guess_the_number();
}
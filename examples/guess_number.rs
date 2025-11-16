use std::{cmp::Ordering, io};
use rand::Rng;

fn guess_numbers() {
    println!("Hello guesser, try a number!");

    loop {
        // #region --- Random number ---
    let mut rnd = rand::rng();
    let random_value = rnd.random_range(1..=100);
    // #endregion --- Random number ---

    // #region --- Read user input ---
    let mut guess_user = String::new();
    io::stdin()
        .read_line(&mut guess_user)
        .expect("Error reading guess");
    // #endregion --- Read user input ---

    // #region --- Convert str to num ---
    let guess_user: i32 = guess_user
        .trim()
        .parse()
        .expect("Error converting, this is a number?");
    // #endregion --- Convert str to num ---

    // #region --- Ordering ---
    match guess_user.cmp(&random_value){
        Ordering::Greater => println!("Greater value than: {random_value}"),
        Ordering::Less => println!("Lower value than: {random_value}"),
        Ordering::Equal => {
            println!("You are the best!! {random_value}");
            break;
        },
    }
    // #endregion --- Ordering ---
    }
}

fn main() {
    guess_numbers();
}
use std::io;

// A simple guess game using random numbers
fn guess_game() {
    println!("Guess a number!");

    println!("Please input your guess.");

    let mut g = String::new();

    // Panics on error!
    io::stdin().read_line(&mut g).expect("Failed to read line!");

    println!("You guessed, {}", g);
}

fn main() {
    guess_game();
}

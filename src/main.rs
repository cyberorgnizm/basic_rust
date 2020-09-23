use rand::Rng;
use std::cmp::Ordering;
use std::io;

// A simple guess game using random numbers
fn guess_game() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    let mut g = String::new();

    // Panics on error!
    // io::stdin().read_line(&mut g).expect("Failed to read line!");

    // Handles panic 🐛
    match io::stdin().read_line(&mut g) {
        Ok(_) => println!("Your input {}", g),
        Err(error) => println!("Error!: {}", error),
    }

    println!("You guessed, {}", g);

    // typecast
    let g: u32 = g.trim().parse().expect("Number expected!");

    match g.cmp(&secret_number) {
        Ordering::Greater => println!("Guess value too big!"),
        Ordering::Equal => println!("You win!!!"),
        Ordering::Less => println!("Guess value too small!"),
    }

    println!("Secret number is {}", secret_number);
}

fn main() {
    guess_game();
}

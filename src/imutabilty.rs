pub fn variables() {
    // variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants != immutable variables, anotation required
    const MAX_POINTS: u32 = 100_000;

    println!("{}", MAX_POINTS);

    // Shadowing: re-delcare new variable with same name as previous one
    let name = "John Doe";

    println!("{}", name);

    // effectively change the type of variable but maintain same name
    let name = 7;

    println!("{}", name)
}

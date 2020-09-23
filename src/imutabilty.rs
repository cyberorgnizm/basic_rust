pub fn variables() {
    // variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants != immutable variables, anotation required
    const MAX_POINTS: u32 = 100_000;

    println!("{}", MAX_POINTS);
}

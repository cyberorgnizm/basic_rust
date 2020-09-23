pub fn variables() {
    // Error [🐛], variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

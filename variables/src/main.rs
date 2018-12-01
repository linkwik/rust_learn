fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // Error: cannot assign twice to immutable variable
    // x = 6;
    
    let mut y = 6;
    println!("The value of y is: {}", y);

    y = 10;
    println!("The value of y is: {}", y);
    
    const MAX_POINTS: u32 = 100_100;
    println!("The constants MAX_POINTS is: {}", MAX_POTINTS);



}

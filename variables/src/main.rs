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
    println!("The constants MAX_POINTS is: {}", MAX_POINTS);

    let s = 5;
    
    let s = s + 1;
 
    let s = s * 2;

    println!("The shadowing is: {}", s);


}

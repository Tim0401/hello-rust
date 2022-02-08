fn main() {
    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);     // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

}

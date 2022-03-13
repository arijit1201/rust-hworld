pub fn main() {

    let mut x: i32 = 1;
    println!("the value of x is: {}", x);

    x = 2;
    println!("the value of x is: {}", x);

    let y: bool = true;
    println!("the value of y is {y}");

    let y: bool = false;
    println!("the value of y is {y}");

    const CONSTANT_STRING: &str = "HELLO THERE";

    println!("The value of the string constant is {CONSTANT_STRING}");
}
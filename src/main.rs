mod scalars;
mod variables;

pub fn displ()
{
    println!("i am displaying something");
}
fn main() {
    
    println!("Microsoft Tutorial on the variables");
    println!("######################################");
    variables::main();
    println!("######################################");
    println!("Microsoft Tutorial on the scalars");
    println!("######################################");
    scalars::main();
    println!("######################################");
    displ();

    
}

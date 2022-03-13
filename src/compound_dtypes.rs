pub fn arrstuples() {
    let arr: [u32; 5] = [1, 2, 4, 5, 6];
    let index = 0;
    //let index = "Some text is nothing".len();
    let first_element = arr[index];
    println!("somwething {first_element}");

    let example: (i32, u8, bool) = (1i32, 2, true);
    
    let x = example.2;
    println!("{x}");
}
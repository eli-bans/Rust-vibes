use std::io;
fn main() {

    //arithmetic
    // you can't add different types together
    let mut input = String::new();
    println!("Enter a number ");
    io::stdin().read_line(&mut input).expect("Failed reading input");

    let int_input: u64 = input.trim().parse().unwrap();
    println!("{}",int_input + 2_u64);

}
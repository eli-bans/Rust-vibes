use std:: io;
fn main() {
   // let arr :[u32 ; 4 ]   = [1,2,3,4];
   //  println!("{}",arr[1]);
    let mut input = String :: new(); //declare an empty string
    println!("Almost forgot this thing ");
    println!("Enter an input? ");

    io::stdin().read_line(&mut input).expect("Failed read");
    println!("Here's your string {}",input);
}

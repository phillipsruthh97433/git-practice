use std::io;

fn main() {
    let mut number = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number = number.trim();
    if number.parse::<i32>().is_ok() {
        println!("The number {} is valid", number);
    } else {
        println!("The number {} is not a valid integer", number);
    }
}

use std::io::{stdin,stdout,Write};

fn main() {
    let mut name=String::new();

    print!("Please enter your name: ");

    // Flust stdout
    let _=stdout().flush();

    stdin().read_line(&mut name).expect("Error reading input");

    println!("Hello, {}", name);
}

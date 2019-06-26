use std::io::{stdin,stdout,Write};

fn main() {
    let mut name=String::new();

    print!("Please enter your name: ");

    // Flust stdout
    let _=stdout().flush();

    stdin().read_line(&mut name).expect("Error reading input");

    if let Some('\n')=name.chars().next_back() {
        name.pop();
    }

    if let Some('\r')=name.chars().next_back() {
        name.pop();
    }

    println!("Hello, {}", name);
}

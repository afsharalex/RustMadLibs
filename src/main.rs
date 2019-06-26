use std::io::{stdin,stdout,Write};

fn main() {
    let name = get_input("Please enter your name: ");

    println!("Hello, {}", name);
}

fn get_input(msg: &'static str) -> String {
    print!("{}", msg); // Print message.

    let mut input = get_raw_input();

    input = clean_input(input);

    input // Return cleaned input.
}

fn get_raw_input() -> String {
    let mut input = String::new();

    let _=stdout().flush(); // Flust stdout.

    stdin().read_line(&mut input).expect("Error reading input");

    input // Return raw input.
}

fn clean_input(mut input: String) -> String {
   // Remove newline.
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    // Remove carriage return.
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    input // Return cleaned input.
}

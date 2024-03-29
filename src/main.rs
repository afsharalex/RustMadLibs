use std::io::{stdin,stdout,Write};

struct MadLib {
    name: String,
    subject: String,
    verb: String,
    object: String
}

impl MadLib {
    fn new(name: String, subject: String,
           verb: String, object: String) -> Self {

        MadLib {
            name: name, subject: subject,
            verb: verb, object: object
        }
    }

    fn create() -> Self {
        let name = get_input("Please enter your name: ");
        let subject = get_input("Please enter a subject: ");
        let verb = get_input("Please enter a verb (past tense): ");
        let object = get_input("Please enter an object: ");

        MadLib::new(name, subject, verb, object)
    }

    fn print(&self) {
        println!("{name} {verb} the {subject} with their {object}",
                 name=self.name,
                 verb=self.verb,
                 subject=self.subject,
                 object=self.object);
    }
}

/*
 * Takes a static str `msg`
 * Returns input from user console.
**/
fn get_input(msg: &'static str) -> String {
    let mut input = String::new();

    print!("{}", msg); // Print message.

    let _=stdout().flush(); // Flush stdout.

    stdin().read_line(&mut input).expect("Error reading input");

    clean_input(&mut input);

    input // Return cleaned input.
}

/*
 * Takes a mutable String reference `input`
 * Cleans String of end chars:
 * `\n` and `\r`.
**/
fn clean_input(input: &mut String) {
   // Remove newline.
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    // Remove carriage return.
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
}


fn main() {
    let ml = MadLib::create();

    ml.print();
}

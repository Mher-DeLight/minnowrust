use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        print!("minnowop> ");
        io::stdout().flush().expect("Failed to flush stdout");

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            break;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }

        match minnowop::evaluate(input) {
            Ok(result) => println!("{result}"),
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }
}

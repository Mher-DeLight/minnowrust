use std::io::{self, Write};

fn main() {
    question_loop();
}

fn question_loop() {
    let mut keychain = minnowop::Keychain::new();
    // i thought of changing this line to use the new API
    // but functional.rs errors when i remove this line for some reason????
    keychain.define_variable('a', minnowop::Operation::Number(5.0));

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

        match minnowop::eval_for(input, &keychain) {
            Ok(result) => println!("{result}"),
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }
}

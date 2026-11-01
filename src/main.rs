use std::io::{self, Write};

use minnowop::{Function, new_keychain};

fn main() {
    question_loop();
}

fn question_loop() {
    let keychain = new_keychain!(a = 5.0, pi = 3.14, e = 2.71, g_earth = 9.81);

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

        let func_res = Function::try_new(input);
        let func: Function;
        match func_res {
            Ok(f) => func = f,
            Err(err) => {
                eprintln!("Parsing Error: {err:?}");
                continue;
            }
        }
        match func.eval_for(&keychain) {
            Ok(result) => println!("{result}"),
            Err(error) => eprintln!("Evaluation Error: {error:?}"),
        }
    }
}

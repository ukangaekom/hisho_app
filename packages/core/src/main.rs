use std::io::{self, BufRead, Write};

use hisho_core::{execute, CommandRequest};

fn main() {
    println!("hisho-core chat started. Type /quit to exit.");
    print!("> ");
    io::stdout().flush().expect("failed to flush stdout");

    for line in io::stdin().lock().lines() {
        let command = match line {
            Ok(command) => command,
            Err(error) => {
                eprintln!("input error: {error}");
                break;
            }
        };

        if command.trim() == "/quit" {
            break;
        }

        match execute(CommandRequest { command }) {
            Ok(response) => println!("{}", response.output),
            Err(error) => println!("error: {error}"),
        }

        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
    }
}
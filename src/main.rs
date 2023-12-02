mod tsh;
use tsh::SimpleShell;
use std::io::{self, Write};

fn main() {
    let shell = SimpleShell::new();

    loop {
        print!("tsh> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if !input.trim().is_empty() {
                let cmd_tokens = shell.parse_command(&input.trim());

                if shell.is_quit(&cmd_tokens[0]) {
                    break;
                }

                shell.exec_command(&cmd_tokens);
            }
        }
    }

    println!();
}


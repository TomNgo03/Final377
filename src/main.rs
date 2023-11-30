use std::io::{self, Write};
use std::process::Command;

struct SimpleShell;

impl SimpleShell {
    fn new() -> SimpleShell {
        return SimpleShell
    }

    fn parse_command(&self, cmd: &str) -> Vec<String> {
        return cmd.split_whitespace().map(String::from).collect()
    }

    fn exec_command(&self, argv: &[String]) {
        match Command::new(&argv[0])
            .args(&argv[1..])
            .status() {
            Ok(_) => {},
            Err(e) => eprintln!("Command Failed: {}", e),
        }
    }

    fn is_quit(&self, cmd: &str) -> bool {
        return cmd == "quit"
    }
}

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


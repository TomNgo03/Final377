mod tsh;
use tsh::SimpleShell;
use std::io::{self, Write};

fn main() {
    let shell = SimpleShell::new();

    loop {
        print!("tsh> ");
        io::stdout().flush().unwrap(); //have to flush output because of the print! (println! automaticlly flushes)

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() { //read_line returns a Result and we check if it's valid
            if !input.trim().is_empty() {
                //taking a command and executing it if it's not `quit`

                let cmd_tokens = shell.parse_command(&input.trim());

                if shell.is_quit(&cmd_tokens[0]) {
                    break;
                }

                let _ = shell.exec_command(&cmd_tokens);
            }
        } else {
            println!("Error recieving input");
        }
    }

    println!();
}


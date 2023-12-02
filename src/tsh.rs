use std::process::{Command, exit};

pub struct SimpleShell;

impl SimpleShell {
    pub fn new() -> SimpleShell {
        return SimpleShell
    }

    pub fn parse_command(&self, cmd: &str) -> Vec<String> {
        return cmd.split_whitespace().map(String::from).collect()
    }

    pub fn exec_command(&self, argv: &[String]) {
        match &argv[0][..] {
            "cd" => {
                if argv.len() > 1 {
                    match std::env::set_current_dir(&argv[1][..]) {
                        Ok(_) => print!("{}", std::env::current_dir().unwrap().display()),
                        Err(e) => eprintln!("cd failed: {}", e),
                    }
                }
            },
            _ => {
                match Command::new(&argv[0])
                    .args(&argv[1..])
                    .spawn()
                {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(status) => {
                                if !status.success() {
                                    exit(1);
                                }
                            }
                            Err(e) => eprintln!("Failed to wait for command: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to execute command: {}", e),
                }
            },  
        }
    }

    pub fn is_quit(&self, cmd: &str) -> bool {
        return cmd == "quit"
    }
}

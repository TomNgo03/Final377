use std::process::{Command, exit};
use core::str::from_utf8;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_quit(){
        let mut shell = SimpleShell::new();
        assert_eq!(shell.is_quit("quit"), true);
        assert_eq!(shell.is_quit("ls"), false);
    }
    
    #[test]
    fn test_parse_command() {
        let shell = SimpleShell::new();

        let argv = shell.parse_command("echo Hello, World!");
        assert_eq!(argv, vec!["echo", "Hello,", "World!"]);

        let argv = shell.parse_command("echo -n Hello, World!");
        assert_eq!(argv, vec!["echo", "-n", "Hello,", "World!"]);
    }

    #[test]
    fn test_exec_command() {
        let shell = SimpleShell::new();
    
        // Test command execution
        let cmd = shell.parse_command("echo Hello, World!");
        let output = shell.exec_command(&cmd).unwrap();
        assert_eq!(from_utf8(&output.stdout).unwrap().trim(), "Hello, World!");
        assert!(output.status.success());
    
        // Test command with arguments
        let cmd = shell.parse_command("echo -n Hello, World!");
        let output = shell.exec_command(&cmd).unwrap();
        assert_eq!(from_utf8(&output.stdout).unwrap(), "Hello, World!");
        assert!(output.status.success());
    }

    #[test]
    fn test_cd_command() {
        let shell = SimpleShell::new();
        let argv = shell.parse_command("cd /tmp");
        let output = shell.exec_command(&argv).unwrap();
        assert!(output.status.success());
        assert_eq!(from_utf8(&output.stdout).unwrap().trim(), "/tmp");
    }

}
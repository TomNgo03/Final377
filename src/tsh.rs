use std::process::{Command, ExitStatus, Stdio};
use std::io::{self, Read};

pub struct SimpleShell;

impl SimpleShell {
    pub fn new() -> SimpleShell {
        SimpleShell
    }

    pub fn parse_command(&self, cmd: &str) -> Vec<String> {
        cmd.split_whitespace().map(String::from).collect()
    }

    pub fn exec_command(&self, argv: &[String]) -> io::Result<String> {
        match &argv[0][..] {
            "cd" => {
                if argv.len() > 1 {
                    match std::env::set_current_dir(&argv[1]) {
                        Ok(_) => Ok(std::env::current_dir()?.display().to_string()),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "No directory specified"))
                }
            },
            _ => {
                let output = Command::new(&argv[0])
                    .args(&argv[1..])
                    .output()?;

                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Command execution failed"))
                }
            },
        }
    }

    pub fn is_quit(&self, cmd: &str) -> bool {
        cmd == "quit"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_quit() {
        let shell = SimpleShell::new();
        assert!(shell.is_quit("quit"));
        assert!(!shell.is_quit("ls"));
    }

    #[test]
    fn test_parse_command() {
        let shell = SimpleShell::new();

        let argv = shell.parse_command("echo Hello, World!");
        assert_eq!(argv, vec!["echo", "Hello,", "World!"]);
    }

    #[test]
    fn test_exec_command_echo() {
        let shell = SimpleShell::new();

        let argv = shell.parse_command("echo Hello, World!");
        assert_eq!(shell.exec_command(&argv).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_exec_command_cd() {
        let shell = SimpleShell::new();
        let argv = shell.parse_command("cd /tmp");
        shell.exec_command(&argv).unwrap();
        assert_eq!(std::env::current_dir().unwrap().display().to_string(), "/tmp");
    }
}

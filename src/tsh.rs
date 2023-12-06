use std::process::{Command};
use std::path::{ PathBuf};
use std::env;


//basciallly exporting SimpleShell so we can access it from main
pub struct SimpleShell;

impl SimpleShell {
    //constructor
    pub fn new() -> SimpleShell {
        return SimpleShell
    }

    //splits command by whitespace and returns a vector of Strings
    //spilt_whitespace returns an iterator of strs, which we then map to Strings and collect to form a vector
    //str is immutable and static while Strings are defined in the heap and can grow
    pub fn parse_command(&self, cmd: &str) -> Vec<String> {
        return cmd.split_whitespace().map(String::from).collect()
    }

    //receives the parsed command and executes the command
    pub fn exec_command(&self, argv: &[String]) -> Result<(), String> {
        //this is a match statement for the first argument– argv[0][..] is a reference to the whole string in argv[0]
        match &argv[0][..] {
             //if it's cd, we use std::env::set_current_dir, to change the directory
            "cd" => {
                if argv.len() == 2 {
                    let mut directory = match env::current_dir() {
                        Ok(dir) => dir,
                        Err(_) => return Err("Failed to get current directory".to_string()),
                    };
                    if &argv[1][..] == ".." {
                        // Move to the parent directory
                        directory.pop();
                    } else {
                        // Handle other paths (both relative and absolute)
                        let target_path = PathBuf::from(&argv[1]);
                        if target_path.is_relative() {
                            directory.push(target_path);
                        } else {
                            directory = target_path;
                        }
                    }

                    //set_current_dir returns a result so we match and provide options in case of success/failure
                    match std::env::set_current_dir(directory.as_path()) { 
                        Ok(_) => {
                            print!("{} ", std::env::current_dir().unwrap().display());
                            Ok(())
                        },
                        Err(_e) => return Err("cd failed".to_string()),
                    }
                }else{
                    return Err("cd requires exactly one argument".to_string());
                }
            },
            _ => { //if it's any other command, we use Command::new() which is basically fork and exec
                match Command::new(&argv[0]) //&argv[0] is the name of the command
                    .args(&argv[1..]).spawn() //&argv[1..] is the rest of the arguments which serve as the arguments to the command
                {
                    //if the new() was succsesful, we wait for the child
                    Ok(mut child) => { 
                        match child.wait() {
                            Ok(status) => {
                                if !status.success() { //if there is something wrong with the wait, we exit the process
                                    return Err("unexpected error".to_string())
                                }else{
                                    Ok(())
                                }
                            }
                            Err(_e) => return Err("the child process failed".to_string()),
                        }
                    }
                    Err(_e) => return Err("failed to execute the command".to_string()),
                }
            },  
        }
    }

    //just checking if the provided command is "quit"
    pub fn is_quit(&self, cmd: &str) -> bool {
        return cmd == "quit"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_is_quit() {
        let tsh = SimpleShell::new();
        assert!(tsh.is_quit("quit"), "quit is not quit");
        assert!(!tsh.is_quit("ls"), "ls is quit");
    }
    #[test]
    fn test_parse_command() {
        let tsh = SimpleShell::new();
        let argv = tsh.parse_command("echo cs377 is amazing");
        assert_eq!(argv, vec!["echo", "cs377", "is", "amazing"]);
    }

    #[test]
    fn test_exec_command() {
        let tsh = SimpleShell::new();
        let argv = tsh.parse_command("echo CS377 is awesome");
        let result = tsh.exec_command(&argv);
        assert!(result.is_ok(), "execute didn't run");
    }

    #[test]
    fn test_cd() {
        let tsh = SimpleShell::new();
        let initial_dir = env::current_dir().expect("Failed to get current directory");
        let _mkdir = tsh.exec_command(&tsh.parse_command("mkdir tmp"));
        let argv = tsh.parse_command("cd tmp");

        let result = tsh.exec_command(&argv);
        assert!(result.is_ok(), "cd command failed");

        let current_dir = env::current_dir().expect("Failed to get current directory");
        assert_eq!(current_dir, initial_dir.join("tmp"), "Current directory is not /tmp");
       
        let _back = tsh.exec_command(&tsh.parse_command("cd .."));
        let parent_dir = env::current_dir().expect("Failed to get current directory");
        assert_eq!(parent_dir, initial_dir, "Current directory is not /");

        let rmdir_result = tsh.exec_command(&tsh.parse_command("rmdir tmp"));
        assert!(rmdir_result.is_ok(), "Failed to remove directory 'tmp'");
    }

    #[test]
    fn test_invalid_command() {
        let tsh = SimpleShell::new();
        let argv = tsh.parse_command("invalid_command");
        let result = tsh.exec_command(&argv);
        assert!(result.is_err(), "Shell should not execute an invalid command.");
    }

    #[test]
    fn test_invalid_cd_path() {
        let tsh = SimpleShell::new();
        let argv = tsh.parse_command("cd /invalid_path");
        let result = tsh.exec_command(&argv);
        assert!(result.is_err(), "Shell should not change to an invalid directory.");
    }

    #[test]
    fn test_missing_arguments() {
        let tsh = SimpleShell::new();
        let argv = tsh.parse_command("cd");
        let result = tsh.exec_command(&argv);
        assert!(result.is_err(), "Expected an error for missing arguments.");
    }
}

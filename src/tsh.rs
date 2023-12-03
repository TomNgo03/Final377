use std::process::{Command, exit};

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
    pub fn exec_command(&self, argv: &[String]) {
        //this is a match statement for the first argument– argv[0][..] is a reference to the whole string in argv[0]
        match &argv[0][..] {
             //if it's cd, we use std::env::set_current_dir, to change the directory
            "cd" => {
                if argv.len() > 1 {
                    //set_current_dir returns a result so we match and provide options in case of success/failure
                    match std::env::set_current_dir(&argv[1][..]) { 
                        Ok(_) => print!("{} ", std::env::current_dir().unwrap().display()),
                        Err(e) => eprintln!("cd failed: {}", e),
                    }
                }
            },
            _ => { //if it's any other command, we use Command::new() which is basically fork and exec
                match Command::new(&argv[0]) //&argv[0] is the name of the command
                    .args(&argv[1..]) //&argv[1..] is the rest of the arguments which serve as the arguments to the command
                    .spawn()
                {
                    //if the new() was succsesful, we wait for the child
                    Ok(mut child) => { 
                        match child.wait() {
                            Ok(status) => {
                                if !status.success() { //if there is something wrong with the wait, we exit the process
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

    //just checking if the provided command is "quit"
    pub fn is_quit(&self, cmd: &str) -> bool {
        return cmd == "quit"
    }
}

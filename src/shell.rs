use std::env;
use std::fs;
use std::io;

pub struct Shell {
    pub command: String,
}

impl Shell {
    pub fn state(&mut self) -> std::io::Result<()> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Not a command. Enter a command.");

        let a = input.trim().to_string();

        let shh = Shell { command: a };

        match shh.command.trim() {
            "ld" => self.ld(),
            "ls" => self.ls(),
            _ => {
                println!("Enter valid command");
                Ok(())
            }
        }
    }

    pub fn ld(&mut self) -> std::io::Result<()> {
        let path = env::current_dir()?;
        println!("{}", path.display());
        Ok(())
    }

    pub fn ls(&mut self) -> std::io::Result<()> {
        let path = env::current_dir()?;
        println!("Current dir: {}", path.display());

        match fs::read_dir(&path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => println!("{}", entry.path().display()),
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }

        Ok(())
    }
}

use std::env;
use std::fs;
use std::io;
use std::string;
use std::path::Path;
pub struct Shell {
    pub command: String,
}

impl Shell {
    
    pub fn state(&mut self) -> std::io::Result<()> {
        
        loop {
            
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Not a command. Enter a command.");

            let split: Vec<&str> = input.split_whitespace().collect();
            if split.is_empty() {
                continue; // skip empty input
            }

            let com = split[0];
            let args = &split[1..];
            let argu = split[1..].join(" ");
            let a = input.trim().to_string();
            let shh = Shell { command: a }; // not used?

            match com.trim() {
                "ld" => {
                    self.ld();
                    Ok(())
                }
                "ls" => {
                    self.ls();
                    Ok(())
                }
                "mkdir" => {
                    self.mkdir(argu);
                    Ok(())
                }
                "rmdir"=>{
                    self.rmdir(argu);
                    Ok(())
                }
                "echo"=>{
                    self.echo(argu);
                    Ok(())
                }
                "cd"=>{
                    self.cd(argu);
                    Ok(())
                }
                _ => {
                    println!("Enter valid command");
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid command",
                    ))
                }
            }?; // <-- propagate error if any
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
    pub fn mkdir(&mut self, arg : String)-> io::Result<()>{
           let folder_name = arg;
           match fs::create_dir(folder_name) {
               Ok(_)=>{

               }
               Err(e)=>{
                println!("Error processing request");
               }
              
           }
            Ok(())
    }
    pub fn rmdir(&mut self, ar:String) -> io::Result<()>{
        let folder = ar;
        match fs::remove_dir(folder) {
                         Ok(_)=>{

                             
                         }
                         Err(e)=>{
                            println!("Error processing request");
                         }

        }
        Ok(())
    }
    pub fn echo(&mut self, are:String)->io::Result<()>{
             let text = are;
             
             println!("{}", text);
             Ok(())
    }
    pub fn cd(&mut self, path:String)->io::Result<()>{
        let mut  directory = path;
        match env::set_current_dir(&mut directory){
            Ok(_)=>{
            
        }
        Err(e)=>{
            eprintln!("Error {}",e);
        }
        
        }
        Ok(())
    }
    

}

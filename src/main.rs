use std::io;

use crate::shell::Shell;
mod shell;

fn main() {
    Shell::state(&mut Shell {  command: String::new()});
    
    
 }

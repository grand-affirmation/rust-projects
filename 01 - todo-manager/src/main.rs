// todo-manager
// main.rs

use std::{env, process};
use tm::TM;

fn main() 
{
    let args = env::args().collect::<Vec<String>>();
    let mut t_manager = TM::new();

    if let Err(e) = t_manager.parse_args(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

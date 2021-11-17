// todo-manager
// main.rs

use std::env;
use tm::TM;

fn main() 
{
    let args = env::args().collect::<Vec<String>>();
    let t_manager = TM::new();
}

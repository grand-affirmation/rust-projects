// todo-manager
// lib.rs
#![feature(path_try_exists)]

use std::fs;
use std::process::{self, Command};
use home;

struct Todo{desc: String, status: bool}

impl Default for Todo {
    fn default() -> Todo {
        Todo{
            desc: String::new(),
            status: false,
        }
    }
}

pub struct TM
{
    todos: Vec<Todo>,
    save_path: String,
}

impl TM
{
    pub fn new() -> TM {
        match fs::try_exists("~/.config/todo-manager/todos") {
            Ok(r) => {
                if r { 
                    unimplemented!()
                } else {
                    let home_dir = home::home_dir().expect("Unable to get home directory.").
                        as_os_str().to_str().unwrap().to_string();
                    fs::create_dir(format!("{}/.config/todo-manager", home_dir));
                    let t: Vec<Todo> = Vec::new();
                    TM {
                        todos: t,
                        save_path: String::from(format!("{}/.config/todo-manager/todos", home_dir)),
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

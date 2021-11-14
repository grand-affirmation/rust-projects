// todo-manager
// lib.rs
#![feature(path_try_exists)]

use std::fs;
use std::process::{self, Command};

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
                    fs::create_dir("~/.config/todo-manager");
                    println!("Made folder");
                    let t: Vec<Todo> = Vec::new();
                    TM {
                        todos: t,
                        save_path: String::from("~/.config/todo-manager/todos"),
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

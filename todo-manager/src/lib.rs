// todo-manager
// lib.rs
#![feature(path_try_exists)]

use std::fs;
use std::process::{self, Command};
use std::error::Error;
use home;

pub struct TM
{
    todos: Vec<Todo>,
    save_path: String,
}

impl TM
{
    pub fn new() -> TM {
        let home_dir = home::home_dir().expect("Unable to get home directory.").as_os_str().to_str().unwrap().to_string();
        let file_path = String::from(format!("{}/.config/todo-manager/todos", home_dir));

        match fs::try_exists(format!("{}/.config/todo-manager", &home_dir)) {
            Ok(r) => {
                if r {
                    let config_output = fs::read_to_string(&file_path).unwrap();
                    let test = config_output.rsplitn(2, ' ').collect::<Vec<&str>>();
                    println!("{}", config_output);
                    println!("{:?}", test);
                    let t: Vec<Todo> = Vec::new();
                    TM {
                        todos: t,
                        save_path: file_path,
                    }
                } else {
                    fs::create_dir(format!("{}/.config/todo-manager", home_dir));
                    let t: Vec<Todo> = Vec::new();
                    TM {
                        todos: t,
                        save_path: file_path,
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }

    pub fn parse_args(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}

struct Todo{desc: String, status: bool}

impl Default for Todo {
    fn default() -> Todo {
        Todo{
            desc: String::new(),
            status: false,
        }
    }
}

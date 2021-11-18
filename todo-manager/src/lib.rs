// todo-manager
// lib.rs
#![feature(path_try_exists)]

use std::fs;
use std::process;
use std::error::Error;
use home;

pub struct TM
{
    todos: Vec<String>,
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
                    let mut lines: Vec<String> = Vec::new();
                    fs::read_to_string(&file_path).unwrap().lines().for_each(|l| lines.push(l.to_string()));
                    TM {
                        todos: lines, 
                        save_path: file_path,
                    }
                } else {
                    fs::create_dir(format!("{}/.config/todo-manager", home_dir));
                    TM {
                        todos: Vec::new(),
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

    pub fn parse_args(&mut self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        if args.len() == 1 {
            show_help();
            process::exit(1);
        } else if args.len() > 1 && args.len() < 3 {
            self.todo_show();
            process::exit(1);
        }

        let option = args.get(1).unwrap();
        let option2 = args.get(2).unwrap().to_string(); // this is the todo item to be add or index to be remove
        
        if option == &"add".to_string() {
            self.todo_add(option2);
        } else if option == &"remove".to_string() {
            self.todo_remove(option2.parse::<usize>().unwrap());
        } else if option == &"show".to_string() {
            self.todo_show();
        } else { 
            show_help();
            process::exit(1);
        }

        Ok(())
    }

    fn todo_add(&mut self, todo: String) {
        self.todos.push(todo);
        let new_list_of_todo: String = self.todos.join("\n");
        if let Err(e) = fs::write(&self.save_path, new_list_of_todo) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }

    fn todo_remove(&mut self, index: usize) {
        self.todos.remove(index-1); // minus 1 because when the todos are displayed, count starts at 1
    }

    fn todo_show(&self) {
        let mut index = 1;

        println!("TODO LIST\n");
        for todo in self.todos.iter() {
            println!("[{}]  {}", index, todo);
            index += 1;
        }
    }
}

fn show_help()
{
    eprintln!("
Todo Manager 

USAGE
    tm [OPTIONS] [args]

OPTIONS
    add     adds a new todo item
    remove  removes a todo item
    show    shows the list of todos");
}

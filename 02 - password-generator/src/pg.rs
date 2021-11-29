// pg.rs

use rand::prelude::*; // for random number generation
use crate::PG; 
use crate::Ops; 
use std::process; // for *properly* exiting the program when errors are found
use colored::*; // colored text

impl PG
{
    // new is for creating a new PG (the struct in lib.rs)
    // it checks for arguments and what they are
    // after checking the args, it executes the corresponding functions
    pub fn new(args: Vec<String>) -> Result<PG, &'static str> {
        if args.is_empty() {
            return Err("Argument empty, check -h!");
        } else if args.len() > 2 {
            return Err("Too many arguments!");
        }

        let mut parsed_ops: Vec<Ops> = Vec::new(); // collects the enabled options
        let mut l: u16 = 8; // length of the generated password; defaults to 8
        
        // argument parsing
        for a in args.iter() {
            if a.get(0..1).unwrap() == "-" {
                if a.contains('h') { // if the arg is -h, show help
                    show_help();
                    process::exit(1);
                }
                if a.contains('u') { // if the arg has -u, enable option for uppercase
                    if !parsed_ops.contains(&Ops::Uppercase) {
                        parsed_ops.push(Ops::Uppercase);
                    }
                }
                if a.contains('l') { // if the arg has -l, enable option for lowercase 
                    if !parsed_ops.contains(&Ops::Lowercase) {
                        parsed_ops.push(Ops::Lowercase);
                    }
                }
                if a.contains('n') { // if the arg has -n, enable option for numbers
                    if !parsed_ops.contains(&Ops::Number) {
                        parsed_ops.push(Ops::Number);
                    }
                }
                if a.contains('s') { // if the arg has -s, enable option for symbols
                    if !parsed_ops.contains(&Ops::Special) {
                        parsed_ops.push(Ops::Special);
                    }
                }
            } else { // if the argument didn't started with '-', treat it as the length
                let parsed_length = match a.parse::<u16>() { 
                    Ok(l) => l,
                    Err(_) => {return Err("Error getting length.");},
                };
                l = parsed_length
            }
        }

        Ok(PG { // return the successfully created PG
            password: String::new(),
            options: parsed_ops,
            length: l,
        })
    }
    
    // generate_password is for generating password 
    // needs a mutable
    pub fn generate_password(&mut self) -> Result<(), &str> {
        let alphabet_low = "qwertyuiopasdfghjklzxcvbnm"; // the lowercase letters
        let alphabet_up = "QWERTYUIOPASDFGHJKLZXCVBNM"; // the uppercase letters
        let numbers = "0123456789"; // the numbers
        let specials = "*&^%$"; // the special characters
        
        // loop that runs for as long as the length
        // then randomizes through the enabled options from before
        // then see what kind of option we got from that, modify self.password
        // self explanatory
        for _ in 0..self.length {
            let r = self.options.get(thread_rng().gen_range(0..self.options.len())).unwrap();
            match r {
                &Ops::Lowercase => {
                    let i: usize = thread_rng().gen_range(0..alphabet_low.len());
                    self.password.push_str(alphabet_low.get(i..i+1).unwrap());
                },
                &Ops::Uppercase => {
                    let i: usize = thread_rng().gen_range(0..alphabet_up.len());
                    self.password.push_str(alphabet_up.get(i..i+1).unwrap());
                },
                &Ops::Number => {
                    let i: usize = thread_rng().gen_range(0..numbers.len());
                    self.password.push_str(numbers.get(i..i+1).unwrap());
                },
                &Ops::Special => {
                    let i: usize = thread_rng().gen_range(0..specials.len());
                    self.password.push_str(specials.get(i..i+1).unwrap());
                },
            }
        }

        println!("{}", self.password); // print the password to the terminal

        Ok(())
    }
}

fn show_help() 
{
    eprintln!("{}", format!("
    {} - a simple password generator 

    {} 
        pg {} {}

    {} (prepend with -)
        {}   use uppercase letters
        {}   use lowercase letters
        {}   use numbers
        {}   use symbols

    {}
        pg -ulns 18
        pg -ln 20", "pg".bold().blue(), "USAGE".blue(), "[OPTIONS]".green(), "[LENGTH]".green(), "OPTIONS".blue(), "u".green(), "l".green(), "n".green(), "s".green(), "EXAMPLE".blue()));
}

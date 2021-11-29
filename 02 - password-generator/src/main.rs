// main.rs

use std::env::args;
use std::process;
use pg::PG;
use colored::*;

fn main() {
    // gets the arguments of the command
    let args: Vec<String> = args().skip(1).collect(); // skip 1 because the first arg is always the program name
    let try_pg = PG::new(args); // try to make a new password generator
    if let Err(e) = try_pg { // see if that pg is an error (failed to create)
        eprintln!("{}", format!("{}: {}", "Error".red(), e)); // print the error
        process::exit(1); // exit the program
    }
    
    let mut pg = try_pg.unwrap(); // if the code gets here, there was no error, so get the password generator
    if let Err(e) = pg.generate_password() { // generate password and see IF it's returns an error
        eprintln!("{}", format!("{}: {}", "Error".red(), e)); // print that error if it is
        process::exit(1); // exit
    }

    // done
}

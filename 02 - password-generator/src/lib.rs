// lib.rs

mod pg;

// the options that can be enabled
#[derive(Debug, PartialEq)]
enum Ops
{
    Uppercase,
    Lowercase,
    Number,
    Special,
}

// the password generator
pub struct PG 
{
    password: String,
    options: Vec<Ops>,
    length: u16,
}

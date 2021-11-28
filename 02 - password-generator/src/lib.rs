mod pg;

#[derive(Debug)]
enum Ops
{
    Uppercase,
    Lowercase,
    Number,
    Special,
}

pub struct PG 
{
    password: String,
    options: Vec<Ops>,
    length: u16,
}

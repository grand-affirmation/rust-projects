use std::error::Error;

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

impl PG
{
    pub fn new(args: Vec<String>) -> Result<PG, &'static str> {
        if args.is_empty() {
            return Err("Not enough arguments!");
        }

        let mut parsed_ops: Vec<Ops> = Vec::new();
        let mut l: u16 = 8;
        
        // argument parsing
        for a in args.iter() {
            if a.get(0..1).unwrap() == "-" {
                if a.contains('u') { parsed_ops.push(Ops::Uppercase); }
                if a.contains('l') { parsed_ops.push(Ops::Lowercase); }
                if a.contains('n') { parsed_ops.push(Ops::Number   ); }
                if a.contains('s') { parsed_ops.push(Ops::Special  ); }
            } else {
                let parsed_length = a.parse::<u16>();
                if let Err(_) = parsed_length {
                    return Err("Argument invalid. Check -h.");
                } else {
                    l = parsed_length.unwrap();
                }
            }
        }

        println!("{:?}, {}", parsed_ops, l);

        Ok(PG {
            password: String::new(),
            options: parsed_ops,
            length: l,
        })
    }

    fn generate_password(&mut self) -> Result<(), Box<dyn Error>> {
        let alphabet_low = "qwertyuiopasdfghjklzxcvbnm";
        let alphabet_up = "QWERTYUIOPASDFGHJKLZXCVBNM";
        let numbers = "0123456789";
        let specials = "*&^%$";

        unimplemented!();

        Ok(())
    }
}

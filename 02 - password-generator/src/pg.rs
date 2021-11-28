use rand::prelude::*;
use crate::PG;
use crate::Ops;

impl PG
{
    pub fn new(args: Vec<String>) -> Result<PG, &'static str> {
        if args.is_empty() {
            return Err("Argument empty, check -h!");
        } else if args.len() > 2 {
            return Err("Too many arguments!");
        }

        let mut parsed_ops: Vec<Ops> = Vec::new();
        let mut l: u16 = 8;
        
        // argument parsing
        for a in args.iter() {
            if a.get(0..1).unwrap() == "-" {
                if a.contains('u') {
                    if !parsed_ops.contains(&Ops::Uppercase) {
                        parsed_ops.push(Ops::Uppercase);
                    }
                }
                if a.contains('l') {
                    if !parsed_ops.contains(&Ops::Lowercase) {
                        parsed_ops.push(Ops::Lowercase);
                    }
                }
                if a.contains('n') {
                    if !parsed_ops.contains(&Ops::Number) {
                        parsed_ops.push(Ops::Number);
                    }
                }
                if a.contains('s') {
                    if !parsed_ops.contains(&Ops::Special) {
                        parsed_ops.push(Ops::Special);
                    }
                }
            } else {
                let parsed_length = a.parse::<u16>();
                if let Err(_) = parsed_length {
                    return Err("Argument invalid. Check -h.");
                } else {
                    l = parsed_length.unwrap();
                }
            }
        }

        Ok(PG {
            password: String::new(),
            options: parsed_ops,
            length: l,
        })
    }

    pub fn generate_password(&mut self) -> Result<(), &str> {
        let alphabet_low = "qwertyuiopasdfghjklzxcvbnm";
        let alphabet_up = "QWERTYUIOPASDFGHJKLZXCVBNM";
        let numbers = "0123456789";
        let specials = "*&^%$";
        
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

        println!("{}", self.password);

        Ok(())
    }
}

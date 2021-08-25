use core::num;
use std::collections::HashMap;

#[derive(Debug)]
enum Map<'a> {
    Value(&'a HashMap<u8, Map<'a>>),
    Nil,
}

enum Mapa {
    Value(Vec<Mapa>),
    Nil
}


impl<'a> Map<'a> {
    fn print(self) {
        if let Value(h) = self {
            for (number , map) in h {
                print!("{} ->", number);
                map.print();
            }
            
        }
    }

    fn check(self, v: &str) -> bool {
        if let Value(h) = self {
            let c = v.chars().next().expect("Error the string is empty");
            let number: u8 = c.to_digit(10).expect("Error converting char to int") as u8;

            print!("{}", number);

            match h.get(&number) {
                Some(next) => {
                    let t = &v[1..];
                    next.check(t);
                }
                None => ()
            }
        }
        else if let Nil = self {
            ()
        }
        false
    }
}


use Map::{Value, Nil};

fn main() {
    println!("Backend Challenge 2021!");

    let empty = HashMap::new();
    let list = Value(&empty);
    
    if let Value(h) = list {
        let mut h3 = HashMap::new();
        h3.insert(4, Nil);

        h.insert(2, Value(&h3));
    }


    println!("end");

}

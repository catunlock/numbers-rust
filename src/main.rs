use std::io::{self, BufRead, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    println!("Backend Challenge 2021!");

    let mut numbers = HashSet::new();

    println!("Enter the allowed numbers (-1 to exit):");
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,    // with ^Z
            Ok(s) => {
                let i : i64 = s.parse().expect("Error converting input to index position.");
                println!("Echo {}", i);
                if i >= 0 {
                    numbers.insert(i);
                }
                else {
                    break;
                }
            }
        }
    }
    
    for n in numbers {
        print!("{}, ", n);
    }

    println!("end");

    Ok(())
}

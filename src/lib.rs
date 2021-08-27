use core::{num, panic};
use std::collections::{HashMap, HashSet};
use std::fmt::Result;
use std::net::{Shutdown, TcpStream};
use std::io::{self, prelude::*};
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn client(addr: &str, start: u32, end: u32, increment: u32) -> io::Result<()> {
    for i in start..end {
        let mut stream = TcpStream::connect(addr)?;
        
        let i = (i * increment).to_string();
        //let i = format!("{:0width$}", i, width = 9);
        
        println!("{}", i);
        stream.write(i.as_bytes())?;
    }   
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn check_range(filename: &str, start: u32, end: u32) -> io::Result<()> {

    let mut numbers = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                let number = number.parse::<u32>().unwrap();
                match numbers.get(&number) {
                    Some(value) => { numbers.insert(number, value + 1); },
                    None => { numbers.insert(number, 1); },
                }
            }
        }
    }

    for i in start..end {
        match numbers.get(&i) {
            Some(value) => {
                if *value > 1 {
                    panic!("number in file at least twice {}", i)
                }
            },
            None => panic!("number missing {}",i),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{io, thread, time::Duration};
    use std::process::Command;


    use threadpool::ThreadPool;

    use crate::{check_range, client};

    const ADDRESS : &str = "localhost:4000";
    const NUMBERS_PATH : &str = "numbers.log";

    #[test]
    fn single_client() -> io::Result<()> {
        client(ADDRESS, 0, 100000, 1)?;
        thread::sleep(Duration::from_secs(2));
        check_range(NUMBERS_PATH, 0, 100000)?;

        Command::new("python")
            .args(&["terminate.py"])
            .output()
            .expect("failed to execute process");
        Ok(())
    }
    
    #[test]
    fn single_client_fail() -> io::Result<()> {
        client(ADDRESS, 0, 100000, 1)?;
        thread::sleep(Duration::from_secs(2));
        check_range(NUMBERS_PATH, 0, 100001)?;

        Command::new("python")
            .args(&["terminate.py"])
            .output()
            .expect("failed to execute process");
        Ok(())
    }

    #[test]
    fn multi_client() -> io::Result<()> {
        
        let pool = ThreadPool::new(5);

        for i in 0..5 {
            let t = pool.execute(move || {
                client(ADDRESS, 100000*i, 100000*(i+1), 1).unwrap();
            });
        }
        pool.join();
        
        
        thread::sleep(Duration::from_secs(2));
        check_range(NUMBERS_PATH, 0, 500000)?;

        Command::new("python")
            .args(&["terminate.py"])
            .output()
            .expect("failed to execute process");
        Ok(())
    }

}
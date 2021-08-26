use std::io::prelude::*;
use std::net::TcpStream;
use std::env;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!("Usage: ./client <ip:port> <start> <end> <increment>");
    }

    let addr = &args[1];
    let start = &args[2].parse::<u32>().expect("start range invalid");
    let end = &args[3].parse::<u32>().expect("start range invalid");
    let increment = &args[4].parse::<u32>().expect("start range invalid");

    for i in *start..*end {
        let mut stream = TcpStream::connect(addr)?;
        let i = (i*increment).to_string();
        //println!("{}", i);
        stream.write(i.as_bytes())?;
    }
    
    Ok(())
} // the stream is closed here
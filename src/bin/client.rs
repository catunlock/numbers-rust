use std::env;

use flap_challenge::client;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: ./client <ip:port> <start> <end> <increment>");
    }

    let addr = &args[1];
    let start = &args[2].parse::<u32>().expect("start range invalid");
    let end = &args[3].parse::<u32>().expect("start range invalid");
    let increment = &args[4].parse::<u32>().expect("start range invalid");

    println!("Start sending numbers from {} to {} in increments of {}", start, end, increment);

    client(addr, *start, *end, *increment)?;

    Ok(())
} // the stream is closed here

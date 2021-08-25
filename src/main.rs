use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;
use std::net::{Shutdown, TcpListener, TcpStream};


fn handle_connection(stream: TcpStream, numbers: &mut HashSet<u64>) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(&stream);

    let mut buf = String::with_capacity(10);
    match reader.read_line(&mut buf) {
        Ok(size_r) => {
            println!("Recv: {:?}, size: {}", buf, size_r);

            match buf.parse::<u64>() {
                Ok(i) => { numbers.insert(i); }
                Err(e) => {println!("Error converting string to int {}", e)}
            }
            
        }
        Err(e) => {
            println!("Error reading from the buffer: {}", e);
        }
    }

    stream.shutdown(Shutdown::Both)
}

fn main() -> io::Result<()> {
    println!("Backend Challenge 2021!");

    let mut numbers: HashSet<u64> = HashSet::new();
    let address = "127.0.0.1:8000";

    println!("Enter the allowed numbers (-1 to exit):");
    
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect(format!("Error creating TCP socket at {}", address).as_str());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        match handle_connection(stream, &mut numbers) {
            Ok(()) => {}
            Err(e) => {println!("Error handling clinet: {}", e)}
        }
    }
    
    for n in numbers {
        print!("{}, ", n);
    }

    println!("end");

    Ok(())
}

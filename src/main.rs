use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::collections::HashSet;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const MAX_CONNECTIONS: i32 = 5;

fn handle_connection(stream: TcpStream, numbers: Arc<Mutex<HashSet<u64>>>) -> Result<bool, std::io::Error> {
    let mut reader = BufReader::new(&stream);

    let mut buf = String::with_capacity(10);
    match reader.read_line(&mut buf) {
        Ok(size_r) => {
            println!("Recv: {:?}, size: {}", buf, size_r);
            
            match buf.parse::<u64>() {
                Ok(i) => { 
                    if numbers.lock().unwrap().insert(i) {
                        // output to numbers.log
                    } 
                }
                Err(e) => {
                    if buf.starts_with("terminate") {
                        return Ok(true);
                    }
                    println!("Error converting string to int {}", e)
                }
            }
        }
        Err(e) => {
            println!("Error reading from the buffer: {}", e);
        }
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        return Err(e);
    }
    Ok(false)
}

fn main() -> io::Result<()> {
    println!("Backend Challenge 2021!");

    let numbers: Arc<Mutex<HashSet<u64>>> = Arc::new(Mutex::new(HashSet::new()));
    let address = "127.0.0.1:8000";

    println!("Enter the allowed numbers (-1 to exit):");
    
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect(format!("Error creating TCP socket at {}", address).as_str());

    let mut n_connections = 0;
    let mut threads: Vec<_> = Vec::new();
    loop {
        let stream = listener.incoming().next().unwrap();
        n_connections += 1;

        match stream {
            Ok(stream) => {
                println!("Connection established!");
                let numbers = Arc::clone(&numbers);
                let connection = thread::spawn(move || handle_connection(stream, numbers));
                threads.push(connection);
            }
            Err(e) => {
                println!("Error listening incomming connection.");
            }
        }

        n_connections -= 1;
    }
    
    for t in threads {
        t.join().unwrap().expect("Thread panicked");
    }

    for n in numbers.lock().unwrap().iter() {
        print!("{}, ", n);
    }

    println!("end");

    Ok(())
}

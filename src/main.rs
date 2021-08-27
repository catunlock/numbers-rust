
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::time::{Duration};

use threadpool::ThreadPool;

const MAX_CONNECTIONS: usize = 24;
const REPORT_TIME: u64 = 1;
const ADDRESS: &str = "127.0.0.1:4000";
const OUT_PATH: &str = "numbers.log";

fn handle_connection(
    stream: TcpStream,
    numbers: Arc<Mutex<HashSet<u64>>>,
    out_file: Arc<Mutex<BufWriter<File>>>,
    duplicates: Arc<Mutex<usize>>,
) -> Result<bool, std::io::Error> {
    let mut reader = BufReader::new(&stream);
    let mut buf = String::with_capacity(10);

    match reader.read_line(&mut buf) {
        Ok(_) => {
            match buf.parse::<u64>() {
                Ok(i) => {
                    //println!("{}", i);

                    if numbers.lock().unwrap().insert(i) {
                        write!(out_file.lock().unwrap(), "{}\n", i)
                            .expect("Error writing number to file.");
                        //println!("new");
                    } else {
                        *duplicates.lock().unwrap() += 1;
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
    /*
        if let Err(e) = stream.shutdown(Shutdown::Both) {
            return Err(e);
        }
    */
    Ok(false)
}

fn report(rx_finish: Receiver<bool>, numbers: &Arc<Mutex<HashSet<u64>>>, duplicates: &Arc<Mutex<usize>>, out_file: &Arc<Mutex<BufWriter<File>>>) {
    let mut prev_unique = 0;
    let mut prev_duplicates = 0;

    loop {
        if let Ok(_) = rx_finish.try_recv() {
            break;
        }
        let uniques = numbers.lock().unwrap().len();
        let duplicates = *duplicates.lock().unwrap();

        let news_unique = uniques - prev_unique;
        let new_duplicates = duplicates - prev_duplicates;
        println!(
            "Received {} unique numbers, {} duplicates. Unique total: {}",
            news_unique, new_duplicates, uniques, 
        );

        prev_unique = uniques;
        prev_duplicates = duplicates;

        out_file.lock().unwrap().flush().unwrap();
        thread::sleep(Duration::from_secs(REPORT_TIME));
    }
}

fn main() -> io::Result<()> {
    println!("Backend Challenge 2021!");

    let out_path = Path::new(OUT_PATH);
    let out_file = BufWriter::new(File::create(&out_path).expect("Error opening creating file."));

    let out_file = Arc::new(Mutex::new(out_file));
    let numbers = Arc::new(Mutex::new(HashSet::new()));
    let duplicates = Arc::new(Mutex::new(0 as usize));

    let listener = TcpListener::bind(ADDRESS)
        .expect(format!("Error creating TCP socket at {}", ADDRESS).as_str());

    let (tx, rx) = channel();
    let (tx_report, rx_report) = channel::<bool>();

    let report_numbers = Arc::clone(&numbers);
    let report_duplicates = Arc::clone(&duplicates);
    let report_outfile = Arc::clone(&out_file);
    let report_thread = thread::spawn(move || 
        report(rx_report, &report_numbers, &report_duplicates, &report_outfile)
    );

    let pool = ThreadPool::new(MAX_CONNECTIONS);

    loop {
        let stream = listener.incoming().next().unwrap();

        match stream {
            Ok(stream) => {
                let numbers = Arc::clone(&numbers);
                let out_file = Arc::clone(&out_file);
                let duplicates = Arc::clone(&duplicates);
                let tx = tx.clone();

                pool.execute(move || {
                    let terminate =
                        handle_connection(stream, numbers, out_file, duplicates).unwrap();
                    if terminate { tx.send(terminate).unwrap(); }
                });
            }
            Err(e) => {
                println!("Error listening incomming connection: {}", e);
            }
        }

        if let Ok(_) = rx.try_recv() {
            break;
        }
    }
    println!("Connection thread end");

    tx_report
        .send(true)
        .expect("Error sending closing message.");

    pool.join();
    report_thread.join().expect("Report thread panicked.");

    out_file.lock().unwrap().flush().unwrap();

    Ok(())
}

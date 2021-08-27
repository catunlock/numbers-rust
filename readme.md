# numbers-rust

A Backend server made in rust that accepts any number of 9 digit numbers as a
single message in each connection, then close the connection. 

The backend must write a de-duplicated list of these numbers to a log file in 
no particular order.

## Build

```
cargo build --release --bin flap_challenge
```

## Run
You can run the binaries in the `target` directory or just run:
```
cargo run --release --bin flap_challenge
```

## Client

To run a client that can connect to the server run:

Usage:
```
cargo run --release --bin client <ip:port> <start> <end> <increment>
```

Example:
```
cargo run --release --bin client localhost:4000 0 1000 1
```

## Tests

The application comes with three test that have to be run individually as each ones closes the server when finish so its a fresh start each time that you run the next test.

All test have into consideration that each number only can appear once in the numbers.log file.

- single_client: A Simple test that runs a client for a range between 0 100000.

- single_client_fail: The same as single_client but checking for one more number that is not in the numbers.log file.

- multi_client: A multi-threaded test that start 5 clients each one with a diferent range and check that all the numbers are in the numbers.log file.
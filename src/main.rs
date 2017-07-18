extern crate prometheus;

use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() {

    let listener = TcpListener::bind("0.0.0.0:5000").unwrap();

    // Create a Counter.
    let counter_opts = Opts::new("test_counter", "test counter help");
    let counter = Counter::with_opts(counter_opts).unwrap();

    // Create a Registry and register Counter.
    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();


    // Output to the standard output.
    // println!("{}", String::from_utf8(buffer).unwrap());

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // handle_client(stream);
                counter.inc();

                let mut buffer = vec![];
                let encoder = TextEncoder::new();
                let metric_familys = r.gather();
                encoder.encode(&metric_familys, &mut buffer).unwrap();
                stream.write(&buffer);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}

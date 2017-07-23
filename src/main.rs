// extern crate prometheus;

// use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
// use std::net::{TcpListener, TcpStream};
// use std::io::Write;
// use std::thread;
use std::str::CharIndices;

fn jaro(s1: &str, s2: &str) -> f64 {
    print!("s1: {} s2: {}\n", s1, s2);
    if s1 == s2 {
        return 1.0;
    }

    let l1 = s1.len();
    let l2 = s2.len();

    let z2 = s2.char_indices();

    // Not sure if I can just drop l1 and l2 into a slice like that.
    // Might not have to subtract the 1 if we're using 0 indexed string.
    let sl = [l1, l2];
    let search_length = (sl.iter().max().unwrap() / 2) - 1;

    let m = s1.char_indices().flat_map(|char_idx| {
        char_match(search_length, z2.clone(), char_idx)
    });
    // .flat_map(|(idx, c)| char_match search_length (idx, c));

    return 10.0;
}

fn char_match(search_length: usize, list: CharIndices, pair: (usize, char)) -> CharIndices {
    let (idx, c) = pair;
    print!("char_match search_length z2 {} {}", idx, c);
    "hello".char_indices()
}

fn main() {
    let res = jaro("hello", "hello");
    print!("result: {}", res);
    // let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
    //
    // // Create a Counter.
    // let counter_opts = Opts::new("test_counter", "test counter help");
    // let counter = Counter::with_opts(counter_opts).unwrap();
    //
    // // Create a Registry and register Counter.
    // let r = Registry::new();
    // r.register(Box::new(counter.clone())).unwrap();
    //
    // // accept connections and process them serially
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             thread::spawn(|| {
    //                 // handle_client(stream);
    //                 counter.inc();
    //
    //                 let mut buffer = vec![];
    //                 let encoder = TextEncoder::new();
    //                 let metric_familys = r.gather();
    //                 encoder.encode(&metric_familys, &mut buffer).unwrap();
    //                 stream.write(&buffer);
    //             });
    //         }
    //         Err(e) => { #<{(| connection failed |)}># }
    //     }
    // }
}

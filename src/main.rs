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

    let sl = [l1, l2];
    // Integer division, not necessary to floor result.
    let match_range = (sl.iter().max().unwrap() / 2) - 1;

    let m: Vec<(usize, char)> = s1.char_indices()
        .flat_map(|idx_char| char_match(match_range, z2.clone(), idx_char))
        .collect();
    let ml = m.len() as f64;

    if ml == 0.0 {
        return 0.0;
    }

    let t: f64 = m.iter().map(|&v| transposition(z2.clone(), v) * 0.5).sum();

    let ml1 = ml / (l1 as f64);
    let ml2 = ml / (l2 as f64);
    let mtm = (ml - t) / ml;

    0.33 * (ml1 + ml2 + mtm)
}

fn transposition(list: CharIndices, (p, q): (usize, char)) -> f64 {
    list.filter(|&(idx, chr)| p != idx && q == chr)
        .collect::<Vec<(usize, char)>>()
        .len() as f64
}

fn char_match(
    match_range: usize,
    list: CharIndices,
    (idx, c): (usize, char),
) -> Vec<(usize, char)> {
    let skip = idx.checked_sub(match_range).unwrap_or(0);
    list.skip(skip)
        .take(idx + match_range)
        .filter(|&(_, chr)| chr == c)
        .collect()
}

extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("jaro string comparison")
        .version("0.1.0")
        .author("stuart nelson <stuartnelson3@gmail.com>")
        .about("compares too strings for similarity")
        .arg(
            Arg::with_name("s1")
                .long("source")
                .value_name("string1")
                .help("the first string")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("s2")
                .long("compare")
                .help("the second string")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let s1 = matches.value_of("s1").unwrap();
    println!("Value for s1: {}", s1);

    let s2 = matches.value_of("s2").unwrap();
    println!("Value for s2: {}", s2);

    let res = jaro(s1, s2);
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

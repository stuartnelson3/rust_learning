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
    // Integer division, not necessary to floor result.
    let match_range = (sl.iter().max().unwrap() / 2) - 1;
    print!("match_range {}\n", match_range);

    let m: Vec<(usize, char)> = s1.char_indices()
        .flat_map(|idx_char| char_match(match_range, z2.clone(), idx_char))
        .collect();
    let ml = m.len() as f64;
    let t: f64 = m.iter().map(|&v| transposition(z2.clone(), v) * 0.5).sum();

    print!("char_match {:?}\n", m);
    let ml1 = ml / (l1 as f64);
    let ml2 = ml / (l2 as f64);
    let mtm = (ml - t) / ml;

    match ml {
        0.0 => 0.0,
        _ => 0.33 * (ml1 + ml2 + mtm),
    }
}

fn transposition(list: CharIndices, pair: (usize, char)) -> f64 {
    let (p, q) = pair;
    let res: Vec<(usize, char)> = list.filter(|idx_char| p != idx_char.0 && q == idx_char.1)
        .collect();
    res.len() as f64
}

fn char_match(match_range: usize, list: CharIndices, pair: (usize, char)) -> Vec<(usize, char)> {
    let (idx, c) = pair;
    let skip = idx.checked_sub(match_range).unwrap_or(0);
    list.skip(skip)
        .take(idx + match_range)
        .filter(|&idx_char| idx_char.1 == c)
        .collect()
}

fn main() {
    let res = jaro("bizmarkie", "zibawekj");
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

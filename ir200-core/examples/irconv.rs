//! Convert a WAV with the library converter and print the 3354 values' bit
//! patterns as JSON (for comparing with tools/explore/irconv.py and slot dumps).
//!
//!     cargo run --release -p ir200-core --example irconv -- IR.wav [channel]
//!     ... --example irconv -- - [channel]     # WAV bytes on stdin

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let data = if a[1] == "-" {
        let mut v = Vec::new();
        std::io::Read::read_to_end(&mut std::io::stdin(), &mut v).expect("read stdin");
        v
    } else {
        std::fs::read(&a[1]).expect("read wav")
    };
    let ch = a.get(2).map_or(0, |c| c.parse().unwrap());
    match ir200_core::irconv::convert_wav(&data, ch) {
        Ok(v) => {
            let w: Vec<String> = v.iter().map(|x| (x.to_bits() as i32).to_string()).collect();
            println!("[{}]", w.join(","));
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
}

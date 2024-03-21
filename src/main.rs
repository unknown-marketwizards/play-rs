#![windows_subsystem = "windows"]

use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return;
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for arg in args.iter().skip(1) {
        let file = BufReader::new(File::open(arg).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }

    sink.sleep_until_end();
}

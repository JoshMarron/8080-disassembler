use std::env;
use std::process;

mod byte_reader;

fn main() {
    let config = byte_reader::Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let reader = byte_reader::ByteReader::new(&config).unwrap_or_else(|err| {
        println!("Error building ByteReader: {}", err);
        process::exit(1);
    });

    println!("{:?}", reader);
}
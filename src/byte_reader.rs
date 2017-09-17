use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    filename: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(fname) => fname,
            None => return Err("Make sure to input a filename")
        };

        Ok(Config {
            filename
        })
    }
}

#[derive(Debug)]
pub struct ByteReader {
    filename: String,
    bytes: Vec<u8>
}

impl ByteReader {
    pub fn new(config: &Config) -> Result<ByteReader, Box<Error>> {
        let mut bytes = Vec::new();
        let mut hex_file = File::open(&config.filename)?;

        hex_file.read_to_end(&mut bytes)?;

        Ok(ByteReader {
            filename: config.filename.clone(),
            bytes
        })
    }
} 
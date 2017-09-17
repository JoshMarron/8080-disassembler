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

pub struct Iter<'a> {
    current: usize,
    vec_ref: &'a Vec<u8>
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.current == self.vec_ref.len() {
            return None
        }

        let item = self.vec_ref[self.current];
        self.current += 1;

        Some(item)
    }
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

    pub fn iter(&self) -> Iter {
        Iter {
            current: 0,
            vec_ref: &self.bytes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_byte_reader_iterator() {
        let mut reader = ByteReader {
            filename: "somename".to_string(),
            bytes: Vec::new()
        };

        reader.bytes.push(12);
        reader.bytes.push(13);
        reader.bytes.push(14);

        let mut iter = reader.iter();

        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(14));
        assert_eq!(iter.next(), None);
    }
}
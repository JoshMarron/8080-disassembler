use byte_reader::ByteReader;
use std::error::Error;

pub fn run(reader: ByteReader) -> Result<(), Box<Error>> {
    disassemble(&reader)?;
    
    Ok(())
}

pub fn disassemble(reader: &ByteReader) -> Result<(), Box<Error>> {
    for byte in reader.iter() {
        println!("{:X}", byte);
    }

    Ok(())
}
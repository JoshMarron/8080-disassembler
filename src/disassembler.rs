use byte_reader::ByteReader;
use std::error::Error;
use std::collections::HashMap;

pub fn run(reader: ByteReader) -> Result<(), Box<Error>> {
    disassemble(&reader)?;
    
    Ok(())
}

pub fn disassemble(reader: &ByteReader) -> Result<(), Box<Error>> {
    let disassembler = Disassembler::new();

    let mut iter = reader.iter();

    while let Some(byte) =  iter.next() {
        match byte {
            0x40...0x7f if disassembler.is_mov_register(byte.clone()) => { 
                let (r1_name, r2_name) = disassembler.get_register_names(byte);
                println!("Move from {} to {} ---- opcode: {:08b}", r2_name, r1_name, byte); 
            },
            0x46...0x7e if disassembler.has_destination_register(byte.clone()) => {
                let (r_name, _) = disassembler.get_register_names(byte);

                println!("Move from memory to {} opcode: {:08b}", r_name, byte);
            },
            0x70...0x77 if disassembler.has_source_register(byte.clone()) => {
                let (_, r_name) = disassembler.get_register_names(byte);

                println!("Move to memory from {} opcode: {:08b}", r_name, byte);
            },
            0x06...0x3e if disassembler.has_destination_register(byte.clone()) => {
                let (r_name, _) = disassembler.get_register_names(byte);
                let data = iter.next().unwrap();

                println!("Move {:08b} to {} -- opcode: {:08b}", data, r_name, byte);
            },
            0x36 => {
                let data = iter.next().unwrap();
                println!("Move {:08b} to memory -- opcode: {:08b}", data, byte);
            }
            _ => continue
        }
    }

    Ok(())
}

struct Disassembler {
    register_map: HashMap<u8, &'static str>,
    register_pair_map: HashMap<u8, &'static str>
}

impl Disassembler {
    fn new() -> Disassembler {
        let mut result = Disassembler {
            register_map: HashMap::new(),
            register_pair_map: HashMap::new()
        };

        result.set_up_register_maps();

        result
    }

    fn set_up_register_maps(&mut self) {
        self.register_map.insert(0b111, "A");
        self.register_map.insert(0b000, "B");
        self.register_map.insert(0b001, "C");
        self.register_map.insert(0b010, "D");
        self.register_map.insert(0b011, "E");
        self.register_map.insert(0b100, "H");
        self.register_map.insert(0b101, "L");

        self.register_pair_map.insert(0b00, "B-C");        
        self.register_pair_map.insert(0b01, "D-E");
        self.register_pair_map.insert(0b10, "H-L");
        self.register_pair_map.insert(0b11, "SP");        
    }

    fn is_mov_register(&self, instruction: u8) -> bool {
        let r1_mask = 0b00111000;
        let r2_mask = 0b00000111;
        
        let r1_code = (instruction & r1_mask) >> 3;
        let r2_code = instruction & r2_mask;

        (r1_code != r2_code) && 
        self.register_map.contains_key(&r1_code) && 
        self.register_map.contains_key(&r2_code)
    }

    fn has_destination_register(&self, instruction: u8) -> bool {
        let r1_mask =       0b00111000;
        let suffix_mask =   0b00000111;

        let r1_code = (instruction & r1_mask) >> 3;
        let suffix = instruction & suffix_mask;

        self.register_map.contains_key(&r1_code) && suffix == 0b110
    }

    fn has_source_register(&self, instruction: u8) -> bool {
        let r1_mask = 0b00000111;
        let r1_code = instruction & r1_mask;

        self.register_map.contains_key(&r1_code)
    }

    fn get_register_names(&self, instruction: u8) -> (String, String) {
        let r1_mask = 0b00111000;
        let r2_mask = 0b00000111;

        let r1_code = (instruction & r1_mask) >> 3;
        let r2_code = instruction & r2_mask;

        let r1_name = self.register_map.get(&r1_code).unwrap_or(&"err").to_string();
        let r2_name = self.register_map.get(&r2_code).unwrap_or(&"err").to_string();

        (r1_name, r2_name)
    }
}








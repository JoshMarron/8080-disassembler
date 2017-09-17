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
    let mut pc : u32 = 0;

    while let Some(byte) =  iter.next() {
        match byte {
            0x00 => {
                println!("{:04X} NOP", pc);
            },
            0x40...0x7f if disassembler.is_mov_register(byte.clone()) => { 
                let (r1_name, r2_name) = disassembler.get_register_names(byte);
                println!("{:04X} MOV {}, {}", pc, r2_name, r1_name); 
            },
            0x46...0x7e if disassembler.has_destination_register(byte.clone()) => {
                let (r_name, _) = disassembler.get_register_names(byte);
                println!("{:04X} MOV {}, M", pc, r_name);
            },
            0x70...0x77 if disassembler.has_source_register(byte.clone()) => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} MOV M, {}", pc, r_name);
            },
            0x06...0x3e if disassembler.has_destination_register(byte.clone()) => {
                let (r_name, _) = disassembler.get_register_names(byte);
                let data = iter.next().unwrap();
                println!("{:04X} MVI {}, #${:02x}", pc, r_name, data);
                pc += 1;
            },
            0x36 => {
                let data = iter.next().unwrap();
                println!("{:04X} MVI M, #${:02x}", pc, data);
                pc += 1;
            },
            0x01 | 0x11 | 0x21 | 0x31 => {
                let low_data = iter.next().unwrap();
                let high_data = iter.next().unwrap();
                let rp_name = disassembler.get_register_pair(byte.clone());
                println!("{:04X} LXI {}, #${:02x}{:02x}", pc, rp_name, high_data, low_data);
                pc += 2;
            },
            0x3a => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} LDA ${:04x}", pc, address);
                pc += 2;
            },
            0x32 => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} STA ${:04x}", pc, address);
                pc += 2;
            },
            0x2a => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} LHLD ${:04x}", pc, address);
                pc += 2;
            },
            0x22 => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} SHLD ${:04x}", pc, address);
                pc += 2;
            },
            0x0a | 0x1a  => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} LDAX {}", pc, rp_name);
            },
            0x02 | 0x12 => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} STAX {}", pc, rp_name);
            },
            0xeb => {
                println!("{:04X} XCHG", pc);
            },
            0x86 => {
                println!("{:04X} ADD M", pc);
            },
            0x80...0x87  => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} ADD {}", pc, r_name);
            },
            0xc6 => {
                let data = iter.next().unwrap();
                println!("{:04X} ADI #${:02x}", pc, data);
                pc += 1
            },
            0x8e => {
                println!("{:04X} ADC M", pc);
            },
            0x88...0x8f => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} ADC {}", pc, r_name);
            },
            0xce => {
                let data = iter.next().unwrap();
                println!("{:04X} ACI #${:02x}", pc, data);
                pc += 1;
            },
            0x96 => {
                println!("{:04X} SUB M", pc);
            },
            0x90...0x97 => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} SUB {}", pc, r_name);
            },
            0xd6 => {
                let data = iter.next().unwrap();
                println!("{:04X} SUI #${:02X}", pc, data);
                pc += 1;
            },
            0x9e => {
                println!("{:04X} SBB M", pc);
            },
            0x98...0x9f => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} SBB {}", pc, r_name);
            },
            0xde => {
                let data = iter.next().unwrap();
                println!("{:04X} SBI #${:02X}", pc, data);
            },
            0x34 => {
                println!("{:04X} INR M", pc);
            },
            0x04...0x3b if disassembler.has_destination_register(byte) => {
                let (r_name, _) = disassembler.get_register_names(byte);
                println!("{:04X} INR {}", pc, r_name);
            },
            0x35 => {
                println!("{:04X} DCR M", pc);
            },
            0x05...0x3d if disassembler.has_destination_register(byte) => {
                let (r_name, _) = disassembler.get_register_names(byte);
                println!("{:04X} DCR {}", pc, r_name);
            },
            0x03 | 0x13 | 0x23 | 0x33 => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} INX {}", pc, rp_name);
            }
            _ => println!("unknown")
        }
        pc += 1;
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

        self.register_pair_map.insert(0b00, "B");        
        self.register_pair_map.insert(0b01, "D");
        self.register_pair_map.insert(0b10, "H");
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

    fn get_register_pair(&self, instruction: u8) -> String {
        let rp_mask = 0b00110000;
        let rp_code = (instruction & rp_mask) >> 4;

        self.register_pair_map.get(&rp_code).unwrap_or(&"err").to_string()
    }
}








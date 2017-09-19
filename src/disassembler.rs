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
            0x01 | 0x11 | 0x21 | 0x31 => {
                let low_data = iter.next().unwrap();
                let high_data = iter.next().unwrap();
                let rp_name = disassembler.get_register_pair(byte.clone());
                println!("{:04X} LXI {}, #${:02x}{:02x}", pc, rp_name, high_data, low_data);
                pc += 2;
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
                pc += 1;
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
            },
            0x0b | 0x1b | 0x2b | 0x3b => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} DCX {}", pc, rp_name);
            },
            0x09 | 0x19 | 0x29 | 0x39 => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} DAD {}", pc, rp_name);
            },
            0x27 => {
                println!("{:04X} DAA", pc);
            },
            0xa6 => {
                println!("{:04X} ANA M", pc);
            },
            0xa0...0xa7 => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} ANA {}", pc, r_name);
            },
            0xe6 => {
                let data = iter.next().unwrap();
                println!("{:04X} ANI #${:02X}", pc, data);
                pc += 1;
            },
            0xae => {
                println!("{:04X} XRA M", pc);
            },
            0xa8...0xaf => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} XRA {}", pc, r_name);
            },
            0xee => {
                let data = iter.next().unwrap();
                println!("{:04X} XRI #${:02X}", pc, data);
                pc += 1;
            },
            0xb6 => {
                println!("{:04X} ORA M", pc);
            },
            0xb0...0xb7 => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} ORA {}", pc, r_name);
            },
            0xf6 => {
                let data = iter.next().unwrap();
                println!("{:04X} ORI #${:02X}", pc, data);
                pc += 1;
            },
            0xbe => {
                println!("{:04X} CMP M", pc);
            },
            0xb8...0xbf => {
                let (_, r_name) = disassembler.get_register_names(byte);
                println!("{:04X} CMP {}", pc, r_name);
            },
            0xfe => {
                let data = iter.next().unwrap();
                println!("{:04X} CPI #${:02X}", pc, data);
                pc += 1;
            },
            0x07 => {
                println!("{:04X} RLC", pc);
            },
            0x0f => {
                println!("{:04X} RRC", pc);
            },
            0x17 => {
                println!("{:04X} RAL", pc);
            },
            0x1f => {
                println!("{:04X} RAR", pc);
            },
            0x2f => {
                println!("{:04X} CMA", pc);
            },
            0x3f => {
                println!("{:04X} CMC", pc);
            },
            0x37 => {
                println!("{:04X} STC", pc);
            },
            0xc3 => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} JMP ${:04x}", pc, address);
                pc += 2;
            },
            0xc2 | 0xca | 0xd2 | 0xda |
            0xe2 | 0xea | 0xf2 | 0xfa => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                let cond = disassembler.get_condition(byte);
                println!("{:04X} JMP {} ${:04x}", pc, cond, address);
                pc += 2;
            },
            0xcd => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} CALL ${:04X}", pc, address);
                pc += 2;
            },
            0xc4 | 0xcc | 0xd4 | 0xdc |
            0xe4 | 0xec | 0xf4 | 0xfc => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                let cond = disassembler.get_condition(byte);
                println!("{:04X} CALL {} ${:04x}", pc, cond, address);
                pc += 2;
            },
            0xc9 => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                println!("{:04X} RET ${:04X}", pc, address);
                pc += 2;
            },
            0xc0 | 0xc8 | 0xd0 | 0xd8 |
            0xe0 | 0xe8 | 0xf0 | 0xf8 => {
                let low_addr = iter.next().unwrap();
                let high_addr = (iter.next().unwrap() as u16) << 8;
                let address = low_addr as u16 + high_addr;
                let cond = disassembler.get_condition(byte);
                println!("{:04X} RET {} ${:04x}", pc, cond, address);
                pc += 2;
            },
            0xc7 | 0xcf | 0xd7 | 0xdf |
            0xe7 | 0xef | 0xf7 | 0xff => {
                let (r_name, _) = disassembler.get_register_names(byte);
                println!("{:04X} RST {}", pc, r_name);
            },
            0xe9 => {
                println!("{:04X} PCHL", pc);
            },
            0xc5 | 0xd5 | 0xe5 => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} PUSH {}", pc, rp_name);
            },
            0xf5 => {
                println!("{:04X} PUSH PSW", pc);
            },
            0xc1 | 0xd1 | 0xe1 => {
                let rp_name = disassembler.get_register_pair(byte);
                println!("{:04X} POP {}", pc, rp_name);
            },
            0xf1 => {
                println!("{:04X} POP PSW", pc);
            },
            0xe3 => {
                println!("{:04X} XTHL", pc);
            },
            0xf9 => {
                println!("{:04X} SPHL", pc);
            },
            0xdb => {
                let port = iter.next().unwrap();
                println!("{:04X} IN #${:02X}", pc, port);
                pc += 1;
            },
            0xd3 => {
                let port = iter.next().unwrap();
                println!("{:04X} OUT #${:02X}", pc, port);
                pc += 1;
            },
            0xfb => {
                println!("{:04X} EI", pc);
            },
            0xf3 => {
                println!("{:04X} DI", pc);
            }
            0x76 => {
                println!("{:04X} HLT", pc);
            },
            0x30 | 0xd9 | 0xcb | 0xed | 0xfd => {
                println!("{:04X} NOP", pc);
            }
            _ => println!("{:04X} unknown: {:02X}", pc, byte)
        }
        pc += 1;
    }

    Ok(())
}

struct Disassembler {
    register_map: HashMap<u8, &'static str>,
    register_pair_map: HashMap<u8, &'static str>,
    condition_code_map: HashMap<u8, &'static str>
}

impl Disassembler {
    fn new() -> Disassembler {
        let mut result = Disassembler {
            register_map: HashMap::new(),
            register_pair_map: HashMap::new(),
            condition_code_map: HashMap::new()
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

        self.condition_code_map.insert(0b000, "NZ");        
        self.condition_code_map.insert(0b001, "Z");       
        self.condition_code_map.insert(0b010, "NC");        
        self.condition_code_map.insert(0b011, "C");        
        self.condition_code_map.insert(0b100, "PO");        
        self.condition_code_map.insert(0b101, "PE");        
        self.condition_code_map.insert(0b110, "P");        
        self.condition_code_map.insert(0b111, "M");        
    }

    fn is_mov_register(&self, instruction: u8) -> bool {
        let r1_mask = 0b00111000;
        let r2_mask = 0b00000111;
        
        let r1_code = (instruction & r1_mask) >> 3;
        let r2_code = instruction & r2_mask;
 
        self.register_map.contains_key(&r1_code) && 
        self.register_map.contains_key(&r2_code)
    }

    fn has_destination_register(&self, instruction: u8) -> bool {
        let r1_mask =       0b00111000;

        let r1_code = (instruction & r1_mask) >> 3;

        self.register_map.contains_key(&r1_code)
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

    fn get_condition(&self, instruction: u8) -> String {
        let cond_mask = 0b00111000;
        let cond_code = (instruction & cond_mask) >> 3;

        self.condition_code_map.get(&cond_code).unwrap_or(&"err").to_string()
    }
}








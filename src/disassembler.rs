use byte_reader::ByteReader;
use std::error::Error;
use std::collections::HashMap;

pub fn run(reader: ByteReader) -> Result<(), Box<Error>> {
    disassemble(&reader)?;
    
    Ok(())
}

pub fn disassemble(reader: &ByteReader) -> Result<(), Box<Error>> {
    let mut register_map : HashMap<u8, &str> = HashMap::new();
    set_up_register_map(&mut register_map);

    for byte in reader.iter() {
        match byte {
            0x40...0x7f if is_mov_register(byte.clone(), &register_map) => { 
                let (r1_name, r2_name) = get_register_names(byte, &register_map);
                println!("Move from {} to {} ---- opcode: 0{:b}", r2_name, r1_name, byte); 
            },
            0x46...0x7e if is_move_from_memory(byte.clone(), &register_map) => {
                let (r_name, _) = get_register_names(byte, &register_map);

                println!("Move from memory to {} opcode: 0{:b}", r_name, byte);
            },
            _ => continue
        }
    }

    Ok(())
}

fn set_up_register_map(map: &mut HashMap<u8, &str>) {
    map.insert(0b111, "A");
    map.insert(0b000, "B");
    map.insert(0b000, "C");
    map.insert(0b010, "D");
    map.insert(0b011, "E");
    map.insert(0b100, "H");
    map.insert(0b101, "L");
}

fn is_mov_register(instruction: u8, register_map: &HashMap<u8, &str>) -> bool {
    let r1_mask = 0b00111000;
    let r2_mask = 0b00000111;
    
    let r1_code = (instruction & r1_mask) >> 3;
    let r2_code = instruction & r2_mask;

    (r1_code != r2_code) && register_map.contains_key(&r1_code) && register_map.contains_key(&r2_code)
}

fn is_move_from_memory(instruction: u8, register_map: &HashMap<u8, &str>) -> bool {
    let r1_mask =       0b00111000;
    let suffix_mask =   0b00000111;

    let r1_code = (instruction & r1_mask) >> 3;
    let suffix = instruction & suffix_mask;

    register_map.contains_key(&r1_code) && suffix == 0b110
}

fn get_register_names(instruction: u8, register_map: &HashMap<u8, &str>) -> (String, String) {
    let r1_mask = 0b00111000;
    let r2_mask = 0b00000111;

    let r1_code = (instruction & r1_mask) >> 3;
    let r2_code = instruction & r2_mask;

    let r1_name = register_map.get(&r1_code).unwrap_or(&"err").to_string();
    let r2_name = register_map.get(&r2_code).unwrap_or(&"err").to_string();

    (r1_name, r2_name)
}
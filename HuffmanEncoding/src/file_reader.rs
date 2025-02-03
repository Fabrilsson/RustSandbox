use std::io::{self, Read, Result, Write};
use std::fs::File;
use std::path::Path;

pub fn read_file_to_string(filename: &str) -> Result<String> {
    let file = File::open(filename);

    let mut content = String::new();
    file.unwrap().read_to_string(&mut content)?;

    Ok(content)
}

pub fn write_bits_to_file(bit_string: &str, file_path: &str) -> Result<()> {
    let mut buffer = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for (i, bit_char) in bit_string.chars().enumerate() {
        let bit = match bit_char {
            '0' => 0,
            '1' => 1,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Bit string inválida")),
        };

        current_byte = (current_byte << 1) | bit;
        bit_count += 1;

        // Accumulate 8 bits, pack as a byte
        if bit_count == 8 {
            buffer.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    // Fill the incomplete bits with zeros
    if bit_count > 0 {
        current_byte <<= 8 - bit_count; // Alinha os bits restantes no byte
        buffer.push(current_byte);
    }

    let mut file_name_counter = 0;

    let mut string_file_name = file_path.to_string();

    while Path::new(&string_file_name).exists() {
        file_name_counter += 1;
        string_file_name.push_str(file_name_counter.to_string().as_str());
    }

    // Write to file
    let mut file = File::create(string_file_name)?;
    file.write_all(&buffer)?;

    Ok(())
}
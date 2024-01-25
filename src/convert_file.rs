use crate::convertions::CONVERTIONS;
use std::path::PathBuf;
use std::{
    fs,
    io::{Result, Write},
};

pub fn convert_file(entry_path: &PathBuf, dest_path: PathBuf) -> Result<()> {
    let mut source_file_buffer = fs::read(entry_path)?;
    let mut destination_file = fs::File::create(dest_path)?;

    for convertion in CONVERTIONS {
        convert_buffer(&mut source_file_buffer, convertion.0, convertion.1);
    }

    destination_file.write_all(&source_file_buffer)?;

    Ok(())
}

fn convert_buffer(buffer: &mut Vec<u8>, from: &[u8], to: &[u8]) {
    let mut i = 0;
    while i < buffer.len() {
        if buffer[i..].starts_with(from) {
            buffer.splice(i..i + from.len(), to.iter().cloned());
            i += to.len();
        } else {
            i += 1;
        }
    }
}

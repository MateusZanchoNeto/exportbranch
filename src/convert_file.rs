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

// fn convert_buffer(buffer: &mut Vec<u8>, from: &[u8], to: &[u8]) {
//     let mut index = 0;
//     while let Some(position) = buffer[index..]
//         .windows(from.len())
//         .position(|window| window == from)
//     {
//         let start = index + position;
//         let end = start + from.len();

//         buffer.splice(start..end, to.iter().cloned());

//         index = start + to.len();
//     }
// }

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

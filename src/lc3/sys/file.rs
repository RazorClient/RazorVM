//The first 16 bits of the program file specify the address in memory where the program should start.
//This address is called the origin. It must be read first,
//after which the rest of the data can be read from the file into memory starting at the origin adress
extern crate byteorder;
 
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crate::hardware::memory::Memory;

pub fn read_image(name: &str) -> io::Result<Memory> {
    let file = File::open(name)?;
    let reader = BufReader::new(file);
    read_image_file(reader)
}

fn read_image_file<R>(mut reader: R) -> io::Result<Memory>
where
    R: Read,
{
    // Read the origin (starting address)
    let origin = reader.read_u16::<BigEndian>()? as usize;

    // Read the rest of the file into a buffer
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

        // Check that the file length is a multiple of 2 (for 16-bit instructions)
    if buffer.len() % 2 != 0 {
        return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size is not a multiple of 2 bytes",
        ));
        }
    // ... parse the buffer, fill `memory`, etc.

    Ok(memory)
}

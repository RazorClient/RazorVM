//The first 16 bits of the program file specify the address in memory where the program should start.
//This address is called the origin. It must be read first,
//after which the rest of the data can be read from the file into memory starting at the origin adress

 
use byteorder::{BigEndian, ReadBytesExt,ByteOrder};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crate::lc3::hardware::Memory::{Memory,MEMORY_SIZE};

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

    // Check that the file length is a multiple of 2 i.e. Validate the File Length
    if buffer.len() % 2 != 0 {
        return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size is not a multiple of 2 bytes",
        ));
        }

    //Convert the raw bytes into 16-bit words in big-endian format
    let num_words = buffer.len() / 2;
    let mut words = vec![0u16; num_words];
    BigEndian::read_u16_into(&buffer, &mut words);

    let mut memory = Memory::new();
    //check if the memory is well sized 
    if origin + num_words > MEMORY_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Program is too large to fit in memory",
        ));
    }

    // Write each word into memory starting at `origin`
    for (i, &word) in words.iter().enumerate() {
            memory.write(origin + i, word);
        }


    Ok(memory)
}

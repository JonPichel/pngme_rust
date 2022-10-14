use std::fs::File;
use std::io::{BufWriter, stdout, Write};
use std::str::FromStr;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.path)?;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    if let Some(out) = &args.output_file {
        let mut out = File::create(&out).unwrap();
        out.write_all(&png.as_bytes())?;
    } else {
        let mut writer = BufWriter::new(stdout().lock());
        writer.write_all(&png.as_bytes())?;
        writer.flush()?;
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;
    if let Some(chunk) = png.chunk_by_type(args.chunk_type.as_str()) {
        println!("{}", chunk.data_as_string().unwrap());
    } else {
        println!("Chunk not found");
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<()> {
    dbg!(args);
    let mut png = Png::from_file(&args.path)?;
    let chunk = png.remove_chunk(args.chunk_type.as_str())?;

    if let Ok(message) = chunk.data_as_string() {
        println!("Message removed: {}", message);
    } else {
        println!("Chunk removed: {}", chunk);
    }

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;
    println!("{}", png);
    Ok(())
}

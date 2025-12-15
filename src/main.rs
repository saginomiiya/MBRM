use std::{
    fs::{
	write, File, OpenOptions
	},
    io::{
	self, Read, Write, Seek, SeekFrom
	}
};
use anyhow::{Context, Result};
use clap::Parser;
mod arguments;
use colored::Colorize;

fn main() -> Result<()> {
    let args = arguments::Args::parse();
    let input_file  = File::open(args.input).context("failed to open file");
    let mut buffer = [0; 512];
    let bytes_read = input_file?.read(&mut buffer).context("failed to read file")?.to_string();
    match &buffer[..] {
        [.., a, b] => if a != &85_u8 && b != &170_u8 {
            panic!("invalid MBR signature")
        },
        _ => panic!("invalid MBR signature") 
    };
    if let Some(output) = args.output {
       let mut file = OpenOptions::new()
       	   .read(true)
	   .write(true)
	   .open(output)?;
       file.seek(SeekFrom::Start(0))?;
       file.write_all(&buffer)?;
       file.sync_data()?;
       return Ok(());
    }
    let mbr_data = &mut buffer
        .chunks(args.chunk_bytes);
    for (index, chunk) in mbr_data.enumerate() {
        let hex_dump_offset = index * args.chunk_bytes;
        writeln!(io::stdout(), "{}:\t{}", if !args.no_color { 
            format!("0x{:0>5x}",hex_dump_offset).red().bold() 
        } else { 
            format!("0x{:0>5x}",hex_dump_offset).bold() 
        }, chunk
            .iter()
            .enumerate()
            .map(|(byte_index, &x)| { 
                let character = if x < 32_u8 || !x.is_ascii() { 
                    ".".to_string() 
                } else {
                    (x as char).to_string()
                };
                if !args.no_color {
                    match hex_dump_offset + byte_index {
                        0x00000..0x001BE => character.on_truecolor(178,0,0),
                        0x001BE..0x001FE => character.on_blue(),
                        0x001FE if x == 0x55 => character.on_truecolor(178,0,178),
                        0x001FF if x == 0xAA => character.on_truecolor(178,0,178),
                        _ => character.on_black(),
                    }.to_string()
                } else { character } 
            })
            .collect::<Vec<String>>()
            .join(""))?;
    }
    if args.blake3 {
        let blake3_checksum = blake3::hash(&buffer).to_string();
        writeln!(io::stdout(), "blake3: {}", if !args.no_color { 
            blake3_checksum.red().bold() 
        } else { 
            blake3_checksum.bold() 
        })?;
    }
    writeln!(io::stdout(), "read {} bytes.", if !args.no_color {
        bytes_read.bold().red()
    } else { 
        bytes_read.bold()
    })?;
    Ok(())
} 

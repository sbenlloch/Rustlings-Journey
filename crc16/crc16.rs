use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        process::exit(1);
    }

    let path = &args[1];
    let mut file = File::open(path)?;

    let mut buffer = [0; 1];
    let mut crc: u16 = 0xFFFF;

    while file.read_exact(&mut buffer).is_ok() {
        crc = crc16_update(crc, buffer[0]);
    }

    println!("CRC16 checksum: 0x{:X}", crc);
    Ok(())
}

fn crc16_update(crc: u16, a: u8) -> u16 {
    let mut crc = crc ^ (a as u16);
    for _ in 0..8 {
        if crc & 1 != 0 {
            crc = (crc >> 1) ^ 0xA001;
        } else {
            crc = crc >> 1;
        }
    }
    crc
}

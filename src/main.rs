use std::{fs::File, io::{BufReader, Read}};

fn main() {
    
    let wav_file = File::open("./samples/bad_guitar.wav").expect("File not found.");
    let wav_buf = BufReader::new(wav_file);
    
    let mut wav_bytes: Vec<u8> = Vec::new();

    for byte in wav_buf.bytes() {
        let byte = byte.expect("Error while reading file.");
        wav_bytes.push(byte);
    }

    // Bytes 4-8 of WAV file are for file size
    let file_size = u32::from_ne_bytes(wav_bytes[4..8].try_into().unwrap());
    println!("The file size is {file_size}");

    let format_tag = u16::from_ne_bytes(wav_bytes[20..22].try_into().unwrap());
    println!("Format tag is {format_tag}");

    let num_channels = u16::from_ne_bytes(wav_bytes[22..24].try_into().unwrap());
    println!("Number of channels is {num_channels}");

    let sample_rate = u32::from_ne_bytes(wav_bytes[24..28].try_into().unwrap());
    println!("Sample rate is {sample_rate}");

    let block_align = u16::from_ne_bytes(wav_bytes[32..34].try_into().unwrap());
    println!("Block align is {block_align}");

    let bits_per_sample = u16::from_ne_bytes(wav_bytes[34..36].try_into().unwrap());
    println!("Bits per sample is {bits_per_sample}");

    // Byte 36 onwards is data chunk
    let mut counter = 44;

    while counter < 48 {
        print!("{:08b} ", wav_bytes[counter]);
        counter += 1;
    }

    // TODO: @siddhantbarua @renormalize: Figure out how to interpret data chunks with the padding byte. 
    // ref: https://wavefilegem.com/how_wave_files_work.html
    let first_sample = i32::from_ne_bytes(wav_bytes[44..48].try_into().unwrap());
    println!("First sample is {first_sample}");

}

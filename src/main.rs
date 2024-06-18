use std::{error, fs::File};

pub mod wav;

fn main() -> Result<(), Box<dyn error::Error>> {
    let wav = wav::Wav::new(String::from("./samples/bad_guitar.wav"))?;
    println!("{:#?}", wav);

    // // Byte 36 onwards is data chunk
    // let mut counter = 44;

    // while counter < 48 {
    //     print!("{:08b} ", wav_bytes[counter]);
    //     counter += 1;
    // }

    // // TODO: @siddhantbarua @renormalize: Figure out how to interpret data chunks with the padding byte.
    // // ref: https://wavefilegem.com/how_wave_files_work.html
    // let first_sample = i32::from_ne_bytes(wav_bytes[44..48].try_into().unwrap());
    // println!("First sample is {first_sample}");

    // let

    Ok(())
}

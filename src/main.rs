use std::error;

pub mod wav;

fn main() -> Result<(), Box<dyn error::Error>> {
    let wav = wav::Wav::new(String::from("./samples/bad_guitar.wav"))?;
    println!("{:#?}", wav.samples); // redirect stdout to a file for inspection
    Ok(())
}

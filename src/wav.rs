use std::fs;

mod error;
use error::*;

// TODO: some fields will be removed
#[derive(Default, Debug)]
pub struct FormatChunkFields {
    chunk_id: u32,
    chunk_body_size: u32,
    // Chunk Body
    format_tag: u16,
    pub num_chan: u16,
    pub sample_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

#[derive(Debug)]
pub enum FormatChunk {
    IntegerPCM {
        format_chunk_fields: FormatChunkFields,
    },
    FloatPCM {
        format_chunk_fields: FormatChunkFields,
        extension_size: u16,
        extra_fields: Vec<u8>,
    },
}

impl Default for FormatChunk {
    fn default() -> Self {
        Self::IntegerPCM {
            format_chunk_fields: FormatChunkFields {
                ..Default::default()
            },
        }
    }
}

#[derive(Default, Debug)]
struct FactChunk {
    chunk_id: u32,
    chunk_body_size: u32,
    no_sample_frames: u32,
}

#[derive(Default, Debug)]
pub struct DataChunk {
    pub chunk_id: u32,
    pub chunk_body_size: u32,
    pub sample_data: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct Wav {
    // Riff Chunk fields
    riff_chunk_id: u32,
    riff_chunk_body_size: u32,
    riff_form_type: u32,

    // Format Chunk fields
    pub format_chunk: FormatChunk,

    // Optional Fact Chunk
    pub fact_chunk: Option<FactChunk>,

    pub data_chunk: DataChunk,
}

impl Wav {
    // new validates the input and creates the Wav
    pub fn new(file_name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(file_name)?;

        // Validating the RIFF chunk
        let (riff_chunk_id, riff_chunk_body_size, riff_form_type) =
            validate_riff_chunk(&data[0..12])?;

        // Children chunks

        // Format Chunk, 12..36
        let format_chunk = validate_format_chunk(&data[12..])?;

        // Optional Fact Chunk for Linear PCM, 36..
        let fact_chunk = validate_fact_chunk(&data[36..])?;

        // Data chunks
        // If fact chunk is present, then the file has to be read further in to obtain the data chunk
        let data_chunk_start = match fact_chunk {
            None => 36,
            Some(_) => todo!(),
        };
        let data_chunk = validate_data_chunk(&data[data_chunk_start..])?;

        Ok(Wav {
            riff_chunk_id,
            riff_chunk_body_size,
            riff_form_type,
            format_chunk,
            fact_chunk,
            data_chunk,
        })
    }
}

// validate_riff_chunk validates the riff chunk and returns the ID, Body Size and Form Type of the chunk
// TODO: since the sequence of bytes for the IDs is hardcoded and are read in a little endian way, this is not platform independent. This will fail on big endian systems.
fn validate_riff_chunk(bytes: &[u8]) -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
    // Chunk ID must be "RIFF" which is 0x46464952
    let chunk_id = match u32::from_ne_bytes(bytes[0..4].try_into()?) {
        0x46464952u32 => 0x46464952u32,
        _ => return Err(WavError::FileNotEncodedProperly.into()),
    };

    // Chunk Body Size will be used to read the file contents
    let chunk_body_size = u32::from_ne_bytes(bytes[4..8].try_into()?);

    // Riff Form Type  must be "WAVE" which is 0x45564157
    let form_type = match u32::from_ne_bytes(bytes[8..12].try_into()?) {
        0x45564157u32 => 0x45564157u32,
        _ => return Err(WavError::FileNotEncodedProperly.into()),
    };

    Ok((chunk_id, chunk_body_size, form_type))
}

// validate_format_chunk validates the format chunk and returns the format chunk
// TODO: since the sequence of bytes for the IDs is hardcoded and are read in a little endian way, this is not platform independent. This will fail on big endian systems.
fn validate_format_chunk(bytes: &[u8]) -> Result<FormatChunk, Box<dyn std::error::Error>> {
    // Chunk ID must be 0x20746d66
    let chunk_id = match u32::from_ne_bytes(bytes[0..4].try_into()?) {
        0x20746d66u32 => 0x20746d66u32,
        _ => return Err(error::WavError::FileNotEncodedProperly.into()),
    };

    let format_chunk_fields = FormatChunkFields {
        chunk_id,
        chunk_body_size: u32::from_ne_bytes(bytes[4..8].try_into()?),
        // place holder format tag, which will not be used in the return
        format_tag: 0u16,
        num_chan: u16::from_ne_bytes(bytes[10..12].try_into()?),
        sample_per_sec: u32::from_ne_bytes(bytes[12..16].try_into()?),
        avg_bytes_per_sec: u32::from_ne_bytes(bytes[16..20].try_into()?),
        block_align: u16::from_ne_bytes(bytes[20..22].try_into()?),
        bits_per_sample: u16::from_ne_bytes(bytes[22..24].try_into()?),
    };

    match u16::from_ne_bytes(bytes[8..10].try_into()?) {
        1u16 => Ok(FormatChunk::IntegerPCM {
            format_chunk_fields: FormatChunkFields {
                format_tag: 1u16,
                ..format_chunk_fields
            },
        }),
        3u16 => Ok(FormatChunk::FloatPCM {
            format_chunk_fields: FormatChunkFields {
                format_tag: 3u16,
                ..format_chunk_fields
            },
            // TODO: @renormalize
            extension_size: todo!(),
            extra_fields: todo!(),
        }),
        _ => Err(WavError::FileNotEncodedProperly.into()),
    }
}

// validate_fact_chunk validates the fact chunk and returns the fact chunk
// TODO: since the sequence of bytes for the IDs is hardcoded and are read in a little endian way, this is not platform independent. This will fail on big endian systems.
fn validate_fact_chunk(bytes: &[u8]) -> Result<Option<FactChunk>, Box<dyn std::error::Error>> {
    let chunk_id = u32::from_ne_bytes(bytes[0..4].try_into()?);
    match chunk_id {
        0x74636166u32 => Ok(Some(FactChunk {
            chunk_id,
            chunk_body_size: u32::from_ne_bytes(bytes[4..8].try_into()?),
            no_sample_frames: u32::from_ne_bytes(bytes[8..12].try_into()?),
        })),
        _ => Ok(None),
    }
}

fn validate_data_chunk(bytes: &[u8]) -> Result<DataChunk, Box<dyn std::error::Error>> {
    let chunk_id = u32::from_ne_bytes(bytes[0..4].try_into()?);
    let chunk_body_size = u32::from_ne_bytes(bytes[4..8].try_into()?);
    Ok(DataChunk {
        chunk_id,
        chunk_body_size,
        sample_data: vec![],
    })
}

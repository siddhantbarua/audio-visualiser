use super::error;
pub use super::*;

// validate_riff_chunk validates the riff chunk and returns the ID, Body Size and Form Type of the chunk
// TODO: since the sequence of bytes for the IDs is hardcoded and are read in a little endian way, this is not platform independent. This will fail on big endian systems.
pub fn riff_chunk(bytes: &[u8]) -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
    // Chunk ID must be "RIFF" which is 0x46464952 when considered a little endian integer
    let chunk_id = match u32::from_le_bytes(bytes[0..4].try_into()?) {
        0x46464952u32 => 0x46464952u32,
        _ => return Err(error::WavError::FileNotEncodedProperly.into()),
    };

    // Chunk Body Size will be used to read the file contents
    let chunk_body_size = u32::from_le_bytes(bytes[4..8].try_into()?);

    // Riff Form Type  must be "WAVE" which is 0x45564157 when considered a little endian integer
    let form_type = match u32::from_le_bytes(bytes[8..12].try_into()?) {
        0x45564157u32 => 0x45564157u32,
        _ => return Err(error::WavError::FileNotEncodedProperly.into()),
    };

    Ok((chunk_id, chunk_body_size, form_type))
}

// validate_format_chunk validates the format chunk and returns the format chunk
// TODO: since the sequence of bytes for the IDs is hardcoded and are read in a little endian way, this is not platform independent. This will fail on big endian systems.
pub fn format_chunk(bytes: &[u8]) -> Result<FormatChunk, Box<dyn std::error::Error>> {
    // Chunk ID must be 0x20746d66 when considered a little endian integer
    let chunk_id = match u32::from_le_bytes(bytes[0..4].try_into()?) {
        0x20746d66u32 => 0x20746d66u32,
        _ => return Err(error::WavError::FileNotEncodedProperly.into()),
    };

    let format_chunk_fields = FormatChunkFields {
        _chunk_id: chunk_id,
        _chunk_body_size: u32::from_le_bytes(bytes[4..8].try_into()?),
        // place holder format tag, which will not be used in the return
        _format_tag: 0u16,
        num_chan: u16::from_le_bytes(bytes[10..12].try_into()?),
        sample_per_sec: u32::from_le_bytes(bytes[12..16].try_into()?),
        avg_bytes_per_sec: u32::from_le_bytes(bytes[16..20].try_into()?),
        block_align: u16::from_le_bytes(bytes[20..22].try_into()?),
        bits_per_sample: u16::from_le_bytes(bytes[22..24].try_into()?),
    };

    match u16::from_le_bytes(bytes[8..10].try_into()?) {
        1u16 => Ok(FormatChunk::IntegerPCM {
            format_chunk_fields: FormatChunkFields {
                _format_tag: 1u16,
                ..format_chunk_fields
            },
        }),
        3u16 => Ok(FormatChunk::FloatPCM {
            format_chunk_fields: FormatChunkFields {
                _format_tag: 3u16,
                ..format_chunk_fields
            },
            // TODO: @renormalize
            extension_size: todo!(),
            extra_fields: todo!(),
        }),
        _ => Err(error::WavError::FileNotEncodedProperly.into()),
    }
}

// validate_fact_chunk validates the fact chunk and returns the fact chunk
// TODO: since the sequence of bytes for the IDs is hardcoded and are read in a little endian way, this is not platform independent. This will fail on big endian systems.
pub fn fact_chunk(bytes: &[u8]) -> Result<Option<FactChunk>, Box<dyn std::error::Error>> {
    let chunk_id = u32::from_le_bytes(bytes[0..4].try_into()?);
    match chunk_id {
        0x74636166u32 => Ok(Some(FactChunk {
            _chunk_id: chunk_id,
            _chunk_body_size: u32::from_le_bytes(bytes[4..8].try_into()?),
            _no_sample_frames: u32::from_le_bytes(bytes[8..12].try_into()?),
        })),
        _ => Ok(None),
    }
}

pub fn data_chunk(bytes: &[u8]) -> Result<DataChunk, Box<dyn std::error::Error>> {
    let chunk_id = u32::from_le_bytes(bytes[0..4].try_into()?);
    let chunk_body_size = u32::from_le_bytes(bytes[4..8].try_into()?);
    Ok(DataChunk {
        chunk_id,
        chunk_body_size,
        sample_data: bytes[8..].to_vec(),
    })
}

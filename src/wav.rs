use std::fs;

mod error;
mod sample;
mod validate;

// TODO: some fields will be removed
#[derive(Default, Debug)]
pub struct FormatChunkFields {
    _chunk_id: u32,
    _chunk_body_size: u32,
    // Chunk Body
    _format_tag: u16,
    pub num_chan: u16,
    pub sample_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16, // size in bytes of each sample frame
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

impl FormatChunk {
    fn num_chan(&self) -> u16 {
        match self {
            FormatChunk::IntegerPCM {
                format_chunk_fields: format_chunk,
            } => format_chunk.num_chan,
            _ => todo!(),
        }
    }
    fn block_align(&self) -> u16 {
        match self {
            FormatChunk::IntegerPCM {
                format_chunk_fields: format_chunk,
            } => format_chunk.block_align,
            _ => todo!(),
        }
    }
    fn bits_per_sample(&self) -> u16 {
        match self {
            FormatChunk::IntegerPCM {
                format_chunk_fields: format_chunk,
            } => format_chunk.bits_per_sample,
            _ => todo!(),
        }
    }
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

// the fact chunk indicates the number of frames in the file
#[derive(Default, Debug)]
struct FactChunk {
    _chunk_id: u32,
    _chunk_body_size: u32,
    // Chunk Body
    _no_sample_frames: u32,
}

#[derive(Default, Debug)]
pub struct DataChunk {
    pub chunk_id: u32,
    pub chunk_body_size: u32,
    // Chunk Body
    pub sample_data: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct Wav {
    // Riff Chunk fields
    _riff_chunk_id: u32,
    _riff_chunk_body_size: u32,
    // Chunk Body
    _riff_form_type: u32,

    // Format Chunk fields
    _format_chunk: FormatChunk,

    // Optional Fact Chunk
    _fact_chunk: Option<FactChunk>,

    _data_chunk: DataChunk,

    pub samples: Vec<SampleFrame>,
}

// ChannelSample is the sample for one channel.
// block_align dictates the sample width
#[derive(Debug, Copy, Clone)]
pub enum ChannelSample {
    U8(u8),
    I16(i16),
    I24(i32), // needs
    I32(i32),
}

#[derive(Debug)]
pub enum SampleFrame {
    Mono(ChannelSample),
    Stereo((ChannelSample, ChannelSample)),
    Multi(Vec<ChannelSample>),
}

impl Wav {
    // new validates the input and creates the Wav
    pub fn new(file_name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(file_name)?;

        // Validating the RIFF chunk
        let (riff_chunk_id, riff_chunk_body_size, riff_form_type) =
            validate::riff_chunk(&data[0..12])?;

        // Children chunks

        // Format Chunk, 12..36
        let format_chunk = validate::format_chunk(&data[12..])?;

        // Optional Fact Chunk for Linear PCM, 36..
        let fact_chunk = validate::fact_chunk(&data[36..])?;

        // Data chunks
        // If fact chunk is present, then the file has to be read further in to obtain the data chunk
        let data_chunk_start = match fact_chunk {
            None => 36,
            Some(_) => todo!(),
        };
        let data_chunk = validate::data_chunk(&data[data_chunk_start..])?;

        let sample_frames = create_samples(&format_chunk, &data_chunk)?;

        Ok(Wav {
            _riff_chunk_id: riff_chunk_id,
            _riff_chunk_body_size: riff_chunk_body_size,
            _riff_form_type: riff_form_type,
            _format_chunk: format_chunk,
            _fact_chunk: fact_chunk,
            _data_chunk: data_chunk,
            samples: sample_frames,
        })
    }
}

// create_samples creates the sample frames from the raw data
// TODO: @renormalize add a test for create_samples
fn create_samples(
    format_chunk: &FormatChunk,
    data_chunk: &DataChunk,
) -> Result<Vec<SampleFrame>, Box<dyn std::error::Error>> {
    let num_chan = format_chunk.num_chan();
    let bits_per_sample = format_chunk.bits_per_sample();
    let bytes_per_frame = format_chunk.block_align();
    let data = &data_chunk.sample_data;

    let mut frame_index = 0;
    let mut frames: Vec<SampleFrame> = vec![];

    let add_to_channels = match bits_per_sample {
        8 => sample::add_to_8,
        16 => sample::add_to_16,
        24 => sample::add_to_24,
        32 => sample::add_to_32,
        _ => panic!("shit"),
    };

    let add_to_frame = match num_chan {
        1 => sample::add_to_mono,
        2 => sample::add_to_stereo,
        _ => sample::add_to_multi,
    };

    while frame_index < data.len() {
        // handle one sample frame
        let frame = &data[frame_index..(frame_index + bytes_per_frame as usize)];
        // all channel samples found within the sample frame
        let mut channels: Vec<ChannelSample> = vec![];
        // 1. iterate through the frame with a width of bits_per_sample and fetch the channel sample bytes
        // 2. Complete this for all channels
        let mut channel_index = 0;
        // iterate over all channel samples in the sample frame
        while channel_index < frame.len() {
            // read bits_per_sample
            // Add interpreted channel sample to the vector of channel samples
            add_to_channels(&mut channels, frame)?;
            // move by one channel sample
            channel_index += (bits_per_sample / 8) as usize;
        }
        // add all the channel samples found as the appropriate frame
        add_to_frame(&mut frames, channels);
        frame_index += bytes_per_frame as usize;
    }

    Ok(frames)
}

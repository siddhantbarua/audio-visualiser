use super::*;

pub fn add_to_8(
    channels: &mut Vec<ChannelSample>,
    frame: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    channels.push(ChannelSample::U8(frame[0]));
    Ok(())
}

pub fn add_to_16(
    channels: &mut Vec<ChannelSample>,
    frame: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let data = i16::from_le_bytes(frame[0..2].try_into()?);
    channels.push(ChannelSample::I16(data));
    Ok(())
}

pub fn add_to_24(
    channels: &mut Vec<ChannelSample>,
    frame: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    // extend the sign
    let sign_extend: u8 = if frame[2] & 0x80 != 0 { 0xFF } else { 0x00 };
    let data = i32::from_le_bytes([frame[0], frame[1], frame[2], sign_extend].try_into()?);
    channels.push(ChannelSample::I32(data));
    Ok(())
}

pub fn add_to_32(
    channels: &mut Vec<ChannelSample>,
    frame: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let data = i32::from_le_bytes(frame[0..4].try_into()?);
    channels.push(ChannelSample::I32(data));
    Ok(())
}

pub fn add_to_mono(frames: &mut Vec<SampleFrame>, channels: Vec<ChannelSample>) {
    frames.push(SampleFrame::Mono(channels[0]));
}

pub fn add_to_stereo(frames: &mut Vec<SampleFrame>, channels: Vec<ChannelSample>) {
    frames.push(SampleFrame::Stereo((channels[0], channels[1])));
}

pub fn add_to_multi(frames: &mut Vec<SampleFrame>, channels: Vec<ChannelSample>) {
    frames.push(SampleFrame::Multi(channels));
}

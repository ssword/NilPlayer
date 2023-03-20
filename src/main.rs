use nil_utils::decode::Decoder;
use nil_utils::errors::NilError;
use nil_wav::decode::WavDecoder;
use rodio::{buffer::SampleBuffer, OutputStream};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), NilError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: nilplayer <filename>");
        return Err(NilError::NoFile);
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        return Err(NilError::FileNotFound);

    }

    if !path.is_file() {
        return Err(NilError::NotAFile):
    }

    let mut wav_decoder = WavDecoder::new(path);

    let decode_result = wav_decoder::decode();

    if let Err(error) = decode_result {
        println!("{:?}", error);
        return Err(NilError::DecodingFailed):
    }

    let channel_data = decode_result.unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to open stream");
    
    // f.extend(channel_data.into_iter().map(|i| i as f32 / i16::MAX as f32));

    stream_handle
        .play_raw(SampleBuffer::new(wav_decoder.channels, wav_decoder.sample_rate, channel_data))
        .expect("Failed to play");

    thread::sleep(Duration::from_secs(wav_decoder.track_length as u64));

    Ok(())
}

use anyhow::{Context, Result};
use camino::Utf8Path as Path;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;
use symphonia::core::units::TimeBase;
use symphonia::core::sample::SampleFormat;

use crate::utils::AudioInfo;
use crate::utils::AudioParameters;

pub fn decode_audio(
    audio_path: &Path,
    output_audio_info: &AudioInfo,
) -> Result<(AudioParameters, Vec<u8>)> {
    let file = std::fs::File::open(audio_path)
        .context("Failed to open audio file")?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    let probe = get_probe();
    let mut format = probe.format(
        &Default::default(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ).context("Failed to create format reader")?.format;

    let track = format.default_track().context("No audio track found")?;
    let codec_params = track.codec_params.clone();
    let time_base = codec_params.time_base.unwrap_or(TimeBase::new(1, 44100));

    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .context("Failed to create decoder")?;

    let mut pcm_data = Vec::new();
    
    let orig_sample_rate = codec_params.sample_rate.unwrap_or(44100);
    let orig_sample_format = codec_params.sample_format.unwrap_or(SampleFormat::F32);
    
    println!("Original sample format: {:?}", orig_sample_format);
    println!("Original sample rate: {}", orig_sample_rate);
    println!("Target sample rate: {}", output_audio_info.sample_rate);
    
    let need_resampling = orig_sample_rate != output_audio_info.sample_rate as u32;
    if need_resampling {
        println!("Note: Sample rate conversion from {} to {} is needed but not implemented in this fix", 
                orig_sample_rate, output_audio_info.sample_rate);
    }

    while let Ok(packet) = format.next_packet() {
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                match audio_buf {
                    AudioBufferRef::F32(buf) => {
                        let spec = buf.spec();
                        let channels = spec.channels.count();
                        
                        for frame_idx in 0..buf.frames() {
                            for ch in 0..channels {
                                let sample = buf.chan(ch)[frame_idx];
                                pcm_data.extend_from_slice(&sample.to_le_bytes());
                            }
                        }
                    },
                    AudioBufferRef::S16(buf) => {
                        let spec = buf.spec();
                        let channels = spec.channels.count();
                        
                        for frame_idx in 0..buf.frames() {
                            for ch in 0..channels {
                                let sample = buf.chan(ch)[frame_idx];
                                let f32_sample = (sample as f32) / 32768.0;
                                pcm_data.extend_from_slice(&f32_sample.to_le_bytes());
                            }
                        }
                    },
                    _ => return Err(anyhow::anyhow!("Unsupported sample format")),
                }
            }
            Err(e) => return Err(e).context("Decoding error"),
        }
    }

    let audio_parameters = AudioParameters {
        time_base,
        codecpar: codec_params,
    };
    println!("Time base: {:?}", audio_parameters.time_base);
    println!("PCM data size: {} bytes, which is {} samples at F32 format", 
             pcm_data.len(), pcm_data.len() / 4 / output_audio_info.nb_channels);

    Ok((audio_parameters, pcm_data))
}
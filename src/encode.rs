use anyhow::{anyhow, Context, Result};
use symphonia::core::sample::SampleFormat;
use camino::Utf8Path as Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::utils::AudioInfo;
use crate::utils::AudioParameters;

pub fn encode_pcm_data(
    pcm_data: &[u8],
    pcm_audio_info: &AudioInfo,
    audio_parameters: &AudioParameters,
    output_path: &Path,
) -> Result<()> {
    
    let extension = output_path.extension().unwrap_or("");
    
    let orig_sample_rate = audio_parameters.codecpar.sample_rate.unwrap_or(44100);
    let orig_sample_format = audio_parameters.codecpar.sample_format.unwrap_or(SampleFormat::F32);
    
    println!("Original sample format: {:?}", orig_sample_format);
    println!("Original sample rate: {}", orig_sample_rate);
    println!("Target sample rate: {}", pcm_audio_info.sample_rate);
    println!("PCM data size: {} bytes (which is {} F32 samples)", 
             pcm_data.len(), pcm_data.len() / 4);
    
    match extension {
        "wav" => {
            let mut corrected_audio_info = pcm_audio_info.clone();
            corrected_audio_info.sample_rate = orig_sample_rate as usize;
            encode_wav(pcm_data, &corrected_audio_info, output_path)
        },
        "mp3" | "aac" | "m4a" => {
            let temp_wav_path = output_path.with_extension("temp.wav");
            let mut corrected_audio_info = pcm_audio_info.clone();
            corrected_audio_info.sample_rate = orig_sample_rate as usize;
            encode_wav(pcm_data, &corrected_audio_info, &temp_wav_path)?;
            
            println!("Converting to {} using ffmpeg", extension);
            let sample_rate = orig_sample_rate.to_string();
            let status = std::process::Command::new("ffmpeg")
                .arg("-y")
                .arg("-i")
                .arg(&temp_wav_path)
                .arg("-b:a")
                .arg("320k")
                .arg("-ar") 
                .arg(&sample_rate)
                .arg("-ac") 
                .arg(&pcm_audio_info.nb_channels.to_string())
                .arg(output_path)
                .status()
                .context("Failed to execute ffmpeg")?;
                
            std::fs::remove_file(&temp_wav_path)?;

            if !status.success() {
                return Err(anyhow!("ffmpeg failed with status: {}", status));
            }
            
            Ok(())
        },
        _ => Err(anyhow!("Unsupported output format: {}", extension)),
    }
}

fn encode_wav(
    pcm_data: &[u8],
    pcm_audio_info: &AudioInfo,
    output_path: &Path,
) -> Result<()> {
    let is_f32 = matches!(pcm_audio_info.sample_fmt, SampleFormat::F32);
    if !is_f32 {
        return Err(anyhow!("Expected F32 sample format, got {:?}", pcm_audio_info.sample_fmt));
    }
    
    let file = File::create(output_path).context("Failed to create output file")?;
    let mut writer = BufWriter::new(file);
    
    let sample_rate = pcm_audio_info.sample_rate as u32;
    
    let data_len = pcm_data.len() as u32;
    let channels = pcm_audio_info.nb_channels as u16;
    let bits_per_sample = (pcm_audio_info.sample_size * 8) as u16;
    let byte_rate = sample_rate * channels as u32 * bits_per_sample as u32 / 8;
    let block_align = channels * bits_per_sample / 8;
    
    println!("Writing WAV with sample rate: {}, channels: {}, bits: {}, format: F32", 
             sample_rate, channels, bits_per_sample);
    
    writer.write_all(b"RIFF")?;
    writer.write_all(&(36 + data_len).to_le_bytes())?;
    writer.write_all(b"WAVE")?;
    
    writer.write_all(b"fmt ")?;
    writer.write_all(&16u32.to_le_bytes())?; // Chunk size
    
    writer.write_all(&3u16.to_le_bytes())?;
    
    writer.write_all(&channels.to_le_bytes())?;
    writer.write_all(&sample_rate.to_le_bytes())?;
    writer.write_all(&byte_rate.to_le_bytes())?;
    writer.write_all(&block_align.to_le_bytes())?;
    writer.write_all(&bits_per_sample.to_le_bytes())?;
    
    writer.write_all(b"data")?;
    writer.write_all(&data_len.to_le_bytes())?;
    writer.write_all(pcm_data)?;  // Raw PCM data
    
    writer.flush()?;
    Ok(())
}
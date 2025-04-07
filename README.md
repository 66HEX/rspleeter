# rsplitter: Spleeter with Symphonia

## Overview
This release replaces rsmpeg with Symphonia, creating a pure Rust implementation for the audio processing pipeline while maintaining compatibility with Spleeter's models for audio source separation.

## Key Changes

- **Replaced FFmpeg dependency**: Switched from rsmpeg to Symphonia for audio decoding operations
- **Native WAV encoding**: Implemented a custom Rust-based WAV encoder
- **Hybrid approach**: Uses pure Rust for core operations while retaining FFmpeg only for specific format conversions
- **Improved error handling**: Enhanced context for errors with anyhow

## Features

- **Multiple separation models**: Support for all standard Spleeter models:
  - 2stems (vocals/accompaniment)
  - 4stems (vocals/drums/bass/other)
  - 5stems (vocals/drums/bass/piano/other)
  - 16kHz variants of all models
- **Efficient processing**: Handles audio in 30-second chunks with 5-second overlaps
- **Format flexibility**: 
  - Input: Any format supported by Symphonia
  - Output: WAV (native), MP3/AAC/M4A (via FFmpeg)
- **Original quality preservation**: Maintains original sample rate and audio parameters

## Technical Implementation

- **TensorFlow integration**: Uses the TensorFlow Rust bindings for running Spleeter models
- **Pure Rust audio decoding**: Leverages Symphonia's capabilities for wide format support
- **Custom WAV encoder**: Implements the WAVE file format specification directly
- **Minimal external dependencies**: Only requires FFmpeg for specific output formats

## Requirements

- Rust toolchain
- TensorFlow
- FFmpeg (optional, only for MP3/AAC/M4A output)

## Basic Usage

```bash
cargo xtask run --release -- input.mp3 output_directory
```

## Why This Fork?

This fork aims to reduce external dependencies by leveraging Rust's growing audio ecosystem, making the project easier to compile and deploy across different platforms while maintaining full compatibility with the original rsplitter functionality.

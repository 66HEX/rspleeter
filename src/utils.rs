use symphonia::core::audio::Layout;
use symphonia::core::codecs::CodecParameters;
use symphonia::core::sample::SampleFormat;
use symphonia::core::units::TimeBase;

#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub codec_id: String,
    pub sample_rate: usize,
    pub sample_fmt: SampleFormat,
    pub channel_layout: Layout,
    pub nb_channels: usize,
    pub sample_size: usize,
}

impl AudioInfo {
    #[allow(unused)]
    fn new(params: &CodecParameters) -> Option<Self> {
        let sample_fmt = params.sample_format?;
        let sample_size = match sample_fmt {
            SampleFormat::F32 => 4,
            SampleFormat::F64 => 8,
            SampleFormat::S32 => 4,
            SampleFormat::S16 => 2,
            SampleFormat::S24 => 3,
            SampleFormat::S8 => 1,
            SampleFormat::U8 => 1,
            SampleFormat::U16 => 2,
            SampleFormat::U24 => 3,
            SampleFormat::U32 => 4,
        };
        let channel_layout = params.channel_layout?;
        
        Some(Self {
            codec_id: params.codec.to_string(),
            sample_rate: params.sample_rate? as usize,
            sample_fmt,
            channel_layout,
            nb_channels: channel_layout.into_channels().count(),
            sample_size,
        })
    }

    pub fn new_pcm(sample_rate: usize) -> Self {
        let sample_fmt = SampleFormat::F32;
        let sample_size = 4; // F32 = 4 bytes
        Self {
            codec_id: "pcm_f32le".to_string(),
            sample_rate,
            sample_fmt,
            channel_layout: Layout::Stereo, 
            nb_channels: 2,
            sample_size,
        }
    }
}

#[derive(Debug)]
pub struct AudioParameters {
    pub time_base: TimeBase,
    pub codecpar: CodecParameters,
}

pub struct AudioData {
    pub nb_channels: usize,
    pub sample_rate: usize,
    pub samples: Vec<f32>,
}

impl AudioData {
    pub fn new(samples: Vec<f32>, nb_channels: usize, sample_rate: usize) -> Self {
        Self {
            nb_channels,
            sample_rate,
            samples,
        }
    }
}
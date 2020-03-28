mod allpass;
mod comb_filter;
mod delay;
mod lowpass;
mod lowpass_comb_filter;
mod reverb;

use std::env;
use time_calc::samples_from_ms;

fn load_file(filename: &str, end_padding: usize) -> Result<Vec<f64>, hound::Error> {
    let mut reader = hound::WavReader::open(filename)?;
    let mut samples = reader
        .samples::<i16>()
        .map(|s| (s.unwrap() as f64) / (std::i16::MAX as f64))
        .collect::<Vec<f64>>();
    samples.append(&mut vec![0.; end_padding]);
    Ok(samples)
}

pub fn save_stereo(arr: &Vec<(f64, f64)>, filename: &str) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;
    for s in arr.iter() {
        writer.write_sample((*s).0 as f32).unwrap();
        writer.write_sample((*s).1 as f32).unwrap();
    }
    writer.finalize()
}

fn main() -> Result<(), hound::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rustverb <path/to/wav>");
        println!("       wav file must be 44100kHz, mono, S16_LE");
        return Err(hound::Error::FormatError("Bad arguments"));
    };

    let padding_ms = 2000.;

    let samples = load_file(
        args[1].as_str(),
        samples_from_ms(padding_ms, 44100.) as usize,
    )?;

    let mut reverb = reverb::Reverb::new(44100.);

    let reverb_samples = samples
        .into_iter()
        .map(|s| reverb.next(s))
        .collect::<Vec<(f64, f64)>>();

    save_stereo(
        &reverb_samples,
        format!("{}.rustverb.wav", args[1]).as_str(),
    )?;

    Ok(())
}

#[cfg(test)]
pub mod test_util {
    pub use super::save_stereo;
    use hound;
    use rand::Rng;

    pub fn generate_noise(length: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| rng.gen::<f64>() * 2. - 1.).collect()
    }

    pub fn generate_impulse(length: usize, offset: usize) -> Vec<f64> {
        let mut arr = vec![0.; length];
        arr[offset] = 1.;
        arr
    }

    pub fn save(arr: &Vec<f64>, filename: &str) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        for s in arr.iter() {
            writer.write_sample(*s as f32).unwrap();
        }
        writer.finalize().unwrap();
    }
}

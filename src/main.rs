mod OnePoleLPF;
mod Delay;
mod CombFilter;
mod LPFCombFilter;
mod DelayAPF;
mod Reverb;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
pub mod test_util {
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

    pub fn load_file(filename: &str, end_padding: usize) -> Vec<f64> {
        let mut reader = hound::WavReader::open(filename).unwrap();
        let mut samples = reader.samples::<i16>()
                                .map(|s| (s.unwrap() as f64) / (std::i16::MAX as f64))
                                .collect::<Vec<f64>>();
        samples.append(&mut vec![0.; end_padding]);
        samples
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

    pub fn save_stereo(arr: &Vec<(f64, f64)>, filename: &str) {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        for s in arr.iter() {
            writer.write_sample((*s).0 as f32).unwrap();
            writer.write_sample((*s).1 as f32).unwrap();
        }
        writer.finalize().unwrap();
    }
}
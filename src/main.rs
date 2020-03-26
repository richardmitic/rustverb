mod OnePoleLPF;
mod Delay;
mod CombFilter;
mod LPFCombFilter;
mod DelayAPF;

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
mod OnePoleLPF;
mod Delay;
mod CombFilter;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
pub mod test_util {
    use hound;

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
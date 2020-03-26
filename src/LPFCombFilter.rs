use crate::Delay::Delay;
use crate::OnePoleLPF::OnePoleLPF;

pub struct LPFCombFilter {
    delay: Delay,
    lowpass: OnePoleLPF,
    g: f64
}

impl LPFCombFilter {
    pub fn new(delay_length: usize, g: f64, sample_rate: f64, cutoff: f64) -> LPFCombFilter {
        LPFCombFilter {
            delay: Delay::new(delay_length),
            lowpass: OnePoleLPF::new(sample_rate, cutoff),
            g: g
        }
    }

    pub fn next(&mut self, s: f64) -> f64 {
        let delayed_sample = self.delay.read();
        self.delay.write_and_advance((self.lowpass.next(delayed_sample) * self.g) + s);
        delayed_sample
    }
}


#[cfg(test)]
mod tests {    
    use super::*;
    use crate::test_util::*;

    #[test]
    fn test_comb_filter() {
        let noise = generate_noise(44100);
        save(&noise, "test_lpf_comb_filter_original.wav");
        
        let mut lpfcf = LPFCombFilter::new(20, 0.5, 44100., 400.);
        let filtered: Vec<f64> = noise.into_iter().map(|s| lpfcf.next(s)).collect();
        save(&filtered, "test_lpf_comb_filter_filtered.wav");
    }
}
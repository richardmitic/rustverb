use crate::Delay::Delay;
use crate::OnePoleLPF::OnePoleLPF;
use crate::LPFCombFilter::LPFCombFilter;
use crate::CombFilter::CombFilter;
use crate::DelayAPF::DelayAPF;

fn samples_from_ms(ms: f64, sample_rate: f64) -> usize {
    time_calc::samples_from_ms(ms, sample_rate) as usize
}

pub struct Reverb {
    pre_delay: Delay,
    lpf1: OnePoleLPF,
    apf1: DelayAPF,
    apf2: DelayAPF,
    comb1: CombFilter,
    comb2: CombFilter,
    lpfcomb3: LPFCombFilter,
    lpfcomb4: LPFCombFilter,
    comb5: CombFilter,
    comb6: CombFilter,
    lpfcomb7: LPFCombFilter,
    lpfcomb8: LPFCombFilter,
    c: [f64; 8],
    lpf2: OnePoleLPF,
    lpf3: OnePoleLPF,
    apf3: DelayAPF,
    apf4: DelayAPF
}

impl Reverb {
    pub fn new(sample_rate: f64) -> Reverb {
        let comb_feedback = 0.8;
        Reverb {
            pre_delay: Delay::new(samples_from_ms(40., sample_rate)),
            lpf1: OnePoleLPF::new(sample_rate, sample_rate * 0.45),
            apf1: DelayAPF::new(samples_from_ms(13.28, sample_rate), 0.7),
            apf2: DelayAPF::new(samples_from_ms(23.1, sample_rate), -0.54),
            comb1: CombFilter::new(samples_from_ms(32.31, sample_rate), comb_feedback),
            comb2: CombFilter::new(samples_from_ms(37.11, sample_rate), comb_feedback),
            lpfcomb3: LPFCombFilter::new(samples_from_ms(40.23, sample_rate), comb_feedback, sample_rate, 11000.),
            lpfcomb4: LPFCombFilter::new(samples_from_ms(44.14, sample_rate), comb_feedback, sample_rate, 9000.),
            comb5: CombFilter::new(samples_from_ms(30.47, sample_rate), comb_feedback),
            comb6: CombFilter::new(samples_from_ms(33.88, sample_rate), comb_feedback),
            lpfcomb7: LPFCombFilter::new(samples_from_ms(41.55, sample_rate), comb_feedback, sample_rate, 11000.),
            lpfcomb8: LPFCombFilter::new(samples_from_ms(42.58, sample_rate), comb_feedback, sample_rate, 9000.),
            c: [0.15, -0.15, 0.15, -0.15, 0.15, -0.15, 0.15, -0.15],
            lpf2: OnePoleLPF::new(sample_rate, sample_rate * 0.49),
            lpf3: OnePoleLPF::new(sample_rate, sample_rate * 0.49),
            apf3: DelayAPF::new(samples_from_ms(9.38, sample_rate), -0.6),
            apf4: DelayAPF::new(samples_from_ms(11.0, sample_rate), 0.6)
        }
    }

    pub fn next(&mut self, value: f64) -> (f64, f64) {
        let mut s = self.pre_delay.read();
        self.pre_delay.write_and_advance(value);
        s = self.lpf1.next(s);
        s = self.apf1.next(s);
        s = self.apf2.next(s);
        let l = s;
        let r = s;

        let mut left =  (self.comb1.next(l) * self.c[0]) +
                        (self.comb2.next(l) * self.c[1]) + 
                        (self.lpfcomb3.next(l) * self.c[2]) + 
                        (self.lpfcomb4.next(l) * self.c[3]);

        let mut right = (self.comb5.next(r) * self.c[4]) +
                        (self.comb6.next(r) * self.c[5]) + 
                        (self.lpfcomb7.next(r) * self.c[6]) + 
                        (self.lpfcomb8.next(r) * self.c[7]);

        left = self.lpf2.next(left);
        left = self.apf3.next(left);

        right = self.lpf3.next(right);
        right = self.apf4.next(right);

        (left, right)
    }
}

#[cfg(test)]
mod tests {    
    use super::*;
    use crate::test_util::*;

    #[test]
    fn test_reverb_noise() {
        let noise = generate_noise(44100);
        save(&noise, "test_reverb_noise_original.wav");
        
        let mut reverb = Reverb::new(44100.);
        let filtered: Vec<(f64, f64)> = noise.into_iter().map(|s| reverb.next(s)).collect();
        save_stereo(&filtered, "test_reverb_noise_filtered.wav");
    }

    #[test]
    fn test_reverb_impulse() {
        let noise = generate_impulse(44100 * 2, 0);
        save(&noise, "test_reverb_impulse_original.wav");
        
        let mut reverb = Reverb::new(44100.);
        let filtered: Vec<(f64, f64)> = noise.into_iter().map(|s| reverb.next(s)).collect();
        save_stereo(&filtered, "test_reverb_impulse_filtered.wav");
    }

    #[test]
    fn test_reverb_guitar() {
        let noise = load_file("/Users/richard/Music/samples/guitarmono.wav", 44100);
        save(&noise, "test_reverb_guitar_original.wav");
        
        let mut reverb = Reverb::new(44100.);
        let filtered: Vec<(f64, f64)> = noise.into_iter().map(|s| reverb.next(s)).collect();
        save_stereo(&filtered, "test_reverb_guitar_filtered.wav");
    }
}
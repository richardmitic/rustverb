use crate::delay::Delay;

pub struct CombFilter {
    delay: Delay,
    g: f64,
}

impl CombFilter {
    pub fn new(delay_length: usize, g: f64) -> CombFilter {
        CombFilter {
            delay: Delay::new(delay_length),
            g: g,
        }
    }

    pub fn next(&mut self, s: f64) -> f64 {
        let delayed_sample = self.delay.read();
        self.delay.write_and_advance((delayed_sample * self.g) + s);
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
        save(&noise, "test_comb_filter_original.wav");

        let mut cf = CombFilter::new(20, 0.5);
        let filtered: Vec<f64> = noise.into_iter().map(|s| cf.next(s)).collect();
        save(&filtered, "test_comb_filter_filtered.wav");
    }
}

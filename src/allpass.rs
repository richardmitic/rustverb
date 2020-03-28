use crate::delay::Delay;

pub struct DelayAPF {
    delay: Delay,
    g: f64,
}

impl DelayAPF {
    pub fn new(delay_length: usize, g: f64) -> DelayAPF {
        DelayAPF {
            delay: Delay::new(delay_length),
            g: g,
        }
    }

    pub fn next(&mut self, s: f64) -> f64 {
        let delayed_sample = self.delay.read();
        let next_sample = (delayed_sample * self.g) + s;
        self.delay.write_and_advance(next_sample);
        next_sample * -self.g
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn test_delay_apf() {
        let noise = generate_noise(44100);
        save(&noise, "test_delay_apf_original.wav");

        let mut lpfcf = DelayAPF::new(140, 0.5);
        let filtered: Vec<f64> = noise.into_iter().map(|s| lpfcf.next(s)).collect();
        save(&filtered, "test_delay_apf_filtered.wav");
    }
}

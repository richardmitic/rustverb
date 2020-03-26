extern crate rand;

pub struct OnePoleLPF {
    a0: f64,
    b1: f64,
    z1: f64
}

impl OnePoleLPF {
    pub fn new(sample_rate: f64, cutoff: f64) -> OnePoleLPF {
        let (a0, b1) = OnePoleLPF::calculate_coefficients(sample_rate, cutoff);
        OnePoleLPF {
            a0: a0,
            b1: b1,
            z1: 0.
        }
    }

    pub fn next(&mut self, s: f64) -> f64 {
        let result = (s * self.a0) - (self.z1 * self.b1);
        self.z1 = result;
        result
    }

    fn calculate_coefficients(sample_rate: f64, cutoff: f64) -> (f64, f64) {
        // let a0 = (2. * std::f64::consts::PI * (cutoff / sample_rate)).sin();
        // let b1 = a0 - 1.;
        let theta = 2. * std::f64::consts::PI * (cutoff / sample_rate);
        let gamma = 2. - theta.cos();
        let b1 = (gamma.powf(2.) - 1.).sqrt() - gamma;
        let a0 = 1. + b1;
        println!("{} {}", a0, b1);
        (a0, b1)
    }
}


#[cfg(test)]
mod tests {    
    use super::*;
    use crate::test_util::*;

    #[test]
    fn test_lowpass() {
        let noise = generate_noise(44100);
        save(&noise, "test_lowpass_original.wav");
        
        let mut lpf = OnePoleLPF::new(44100., 400.);
        let filtered: Vec<f64> = noise.into_iter().map(|s| lpf.next(s)).collect();
        save(&filtered, "test_lowpass_filtered.wav");
    }
}
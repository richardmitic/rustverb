pub struct Delay {
    buffer: Vec<f64>,
    index: usize
}

impl Delay {
    pub fn new(length: usize) -> Delay {
        Delay {
            buffer: vec![0.; length],
            index: 0
        }
    }

    pub fn read(&self) -> f64 {
        self.buffer[self.index]
    }
    
    pub fn write_and_advance(&mut self, s: f64) {
        self.buffer[self.index] = s;
        self.index = (self.index + 1) % self.buffer.len();
    }
}

mod tests {    
    use super::*;

    #[test]
    fn test_delay() {
        let mut d = Delay::new(3);
        let results = (1..=10).map(|i| {
            let s = d.read();
            d.write_and_advance(i as f64);
            s
        }).collect::<Vec<f64>>();
        assert_eq!(results, vec![0., 0., 0., 1., 2., 3., 4., 5., 6., 7.]);
    }
}
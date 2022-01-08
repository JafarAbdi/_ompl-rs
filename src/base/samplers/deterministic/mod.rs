use primal;

pub trait DeterministicSequence {
    fn sample(&mut self) -> Vec<f64>;
}

pub struct HaltonSequence {
    index: usize,
    bases: Vec<usize>,
}

impl HaltonSequence {
    pub fn new(bases: Vec<usize>) -> HaltonSequence {
        HaltonSequence { index: 1, bases }
    }

    pub fn from_primes(dimensions: usize) -> HaltonSequence {
        HaltonSequence::new(primal::Primes::all().take(dimensions).collect())
    }

    fn sample_1d(mut index: usize, base: usize) -> f64 {
        let mut f = 1.0;
        let mut r = 0.0;
        while index > 0 {
            f /= base as f64;
            r += f * (index % base) as f64;
            index /= base;
        }
        r
    }
}

impl DeterministicSequence for HaltonSequence {
    fn sample(&mut self) -> Vec<f64> {
        let index = self.index;
        self.index += 1;
        self.bases
            .iter()
            .map(|base| HaltonSequence::sample_1d(index, *base))
            .collect()
    }
}

use super::TEST_RESOURCE_DIR;
use approx::RelativeEq;
use ompl::base::{DeterministicSequence, HaltonSequence};
use std::fs;

fn load_sequence_from_file(filename: String) -> Vec<Vec<f64>> {
    let file = fs::read_to_string(&filename).unwrap();
    file.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn test_halton() {
    for dimension in [1, 2, 5, 10] {
        let sequence: Vec<_> = {
            let mut sequence = HaltonSequence::from_primes(dimension);
            (0..5).map(move |_| sequence.sample()).collect()
        };
        let expected_sequence = load_sequence_from_file(format!(
            "{}/halton/halton_{}d.txt",
            TEST_RESOURCE_DIR, dimension
        ));

        assert!(sequence.concat().relative_eq(
            expected_sequence.concat().as_slice(),
            1e-3,
            f64::default_max_relative()
        ));
    }
}

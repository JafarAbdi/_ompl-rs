use ob::DeterministicSequence;
use ompl::base as ob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let random_points: Vec<Vec<f64>> = {
        let mut sequence = ob::HaltonSequence::from_primes(5);
        (0..5).map(move |_i| sequence.sample()).collect()
    };
    eprintln!("random_points = {:?}", random_points);
    Ok(())
}

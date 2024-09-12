use rdrand::RdRand; 
use crate::Simulate;

pub struct Coin;

impl Simulate for Coin {
    fn simulate_number_events(number: u64) -> f64 {
        let mut rdgen = RdRand::new().unwrap();
        let mut heads: u64 = 0;

        // Lets say heads are true and tails are false
        for index in 0..number {
            if rdgen.try_next_u16().unwrap() % 2 == 0 {
                heads += 1;
            }
        }
        
        heads as f64 / number as f64
        
    }

    fn multiple_experiments(start: u64, limit: u64, step: u64) {
        let real_limit = (limit as f64 / step as f64).floor() as u64;
        let mut number_events;

        for experiment_n in start..=real_limit {
            number_events = experiment_n * step;
            println!("Probability of coin landing head for {} events: {:.6}.", number_events, 
                Coin::simulate_number_events(number_events));
        }
    }
}

mod coin_flip;

fn main() {
    coin_flip::Coin::multiple_experiments(1, 200000, 1000);
}

pub trait Simulate {
    fn simulate_number_events(number: u64) -> f64;
    fn multiple_experiments(start: u64, limit: u64, step: u64);
}

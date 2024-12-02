use rand::{Rng, SeedableRng};

fn main() {
    let state = 0;

    let mut rand = rand::prelude::SmallRng::seed_from_u64(state);
    for index in 0..10 {
        let value = rand.random_range(-1f32..1f32);
        println!("value {}: {}", index, value);
    }
}

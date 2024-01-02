use itertools::Itertools;

fn main() {
    let permutations = (0..8).permutations(8);
    for perm in permutations {
        println!("{:?}", perm);
    }
}

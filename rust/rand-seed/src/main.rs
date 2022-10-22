use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

fn main() {
    let seed = "stocazzo";

    let mut rand1: Pcg64 = Seeder::from(&seed).make_rng();
    let mut rand2: Pcg64 = Seeder::from(&seed).make_rng();

    let num1 = rand1.gen_range(0..100);
    let num2 = rand2.gen_range(0..100);

    println!("{}; {}", num1, num2);
}

use num_bigint::BigUint;
use num_traits::{Zero, One};
use rand::seq::IteratorRandom;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let vs = fibs(50);
    for v in &vs {
        println!("{}", v);
    }
    let selection = vs.iter().choose(&mut rng).unwrap();
    println!("{}", selection);
}

fn fibs(n: u64) -> Vec<BigUint> {
    let mut results = vec![BigUint::zero(), BigUint::one()];
    for _i in 0..n {
        let len = results.len();
        let v = &results[len-2] + &results[len-1];
        results.push(v);
    }
    results.drain(..2);
    results
}

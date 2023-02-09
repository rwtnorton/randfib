use clap::Parser;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use rand::seq::IteratorRandom;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, default_value_t = 20)]
    n: u64,
}

fn main() {
    let mut rng = rand::thread_rng();
    let args = Args::parse();
    let vs = fibs(args.n);
    // for v in &vs {
    //     println!("{}", v);
    // }
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

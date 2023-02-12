use clap::Parser;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, default_value_t = 20)]
    n: u64,
}

fn main() {
    let mut rng = rand::thread_rng();
    let args = Args::parse();
    // Skip over the first two fibonacci values, 0 and 1,
    // to align better with typical Scrum story point values.
    let i: u64 = rng.gen_range(2..(args.n+2));
    let selection = nth_fib(i);
    println!("{}", selection);
}

fn nth_fib(n: u64) -> BigUint {
    let mut f0 = BigUint::zero();
    let mut f1 = BigUint::one();
    for _i in 0..n {
        let f2 = &f0 + &f1;
        f0 = f1.clone();
        f1 = f2;
    }
    f0
}

#[test]
fn test_nth_fib() {
    let test_cases : Vec<(u64, BigUint)>= vec![
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144,
        233, 377, 610, 987, 1597,
    ].iter().enumerate().map(|(i, n)| {
        (i as u64, BigUint::from(n.clone() as u64))
    })
        .collect();
    for (given, expect) in test_cases {
        let got = nth_fib(given);
        assert_eq!(
            got, expect,
            "given: {}, expect: {}, got: {}",
            given, expect, got,
        );
    }
}

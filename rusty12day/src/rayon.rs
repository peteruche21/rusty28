use num::{BigUint, One};
use rayon::prelude::*;
use std::time::Instant;

fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        BigUint::one()
    } else {
        (1..=num)
            .map(BigUint::from)
            .reduce(|acc, x| acc * x)
            .unwrap()
    }
}

fn multi_fact(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        BigUint::one()
    } else {
        (1..=num)
            .into_par_iter()
            .map(BigUint::from)
            .reduce(|| BigUint::one(), |acc, x| acc)
    }
}

pub fn rayon() {
    let instance = Instant::now();

    factorial(50000);

    println!("{:?}", instance.elapsed());

    let instance = Instant::now();

    multi_fact(50000);

    println!("{:?}", instance.elapsed());
}

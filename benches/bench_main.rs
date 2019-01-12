



#[macro_use] extern crate criterion;
extern crate num_bigint_dig as num_bigint;
extern crate rand;
extern crate rand_chacha;
extern crate num_integer;
extern crate num_traits;



mod bmarks;


criterion_main! {
    bmarks::prime_benches::benches,
    bmarks::gcd::benches,
    bmarks::factorial::benches,
    bmarks::bigint::benches,
}



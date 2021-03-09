extern crate rand;

use rand::{thread_rng, Rng};

pub fn find_co_prime(number: u128, max: u128) -> u128 {
  let mut rng = thread_rng();
  let mut candidate = rng.gen_range(2, max);
  while gcd(candidate, number) != 1 {
    candidate = (candidate + 1) % max;
    if candidate <= 2 {
      candidate = 3;
    }
  }
  return candidate;
}

fn gcd(num1: u128, num2: u128) -> u128 {
  let mut gcd = 1;
  for i in 2..num1 {
    if (num1 % i == 0) && (num2 % i == 0) {
      gcd = i;
    }
  }
  return gcd;
}

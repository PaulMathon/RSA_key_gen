extern crate rand;

use rand::Rng;

pub fn gen_prime() -> u64 {
  let mut prime = generate_prime_candidate();
  while !is_prime(prime, 128) {
    prime = generate_prime_candidate();
  }
  return prime;
}

fn is_prime(number: u64, k: u32) -> bool {
  let mut r = 0;
  let mut s = number - 1;
  while s % 2 == 0 {
    r += 1;
    s = s / 2;
  }
  for _ in 0..k {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(2, number - 1);
    let mut x = modpow(a, s, number);
    if x == 1 || x == (number as u128) - 1 {
      for _ in 0..(r - 1) {
        x = modpow((x % (number as u128)) as u64, 2, number);
        if x == (number - 1) as u128 {
          break;
        }
      }
    }
    else {
      return false
    }
  }
  return true;  
}

fn generate_prime_candidate() -> u64 {
  let mut rng = rand::thread_rng();
  let mut candidate:u64 = rng.gen();
  candidate = check_odd(candidate);
  candidate = check_msb(candidate);
  return candidate;
}

fn check_odd(mut number: u64) -> u64 {
  if &number % 2 == 0 {
    number += 1;
  }
  return number;
}

fn check_msb(mut number: u64) -> u64 {
  if number.leading_zeros() > 0 {
    let mut prime_str = String::from("1");
    for _ in 0..(number.leading_zeros() - 1) {
      prime_str.push('0');
    }
    number = u64::from_str_radix(&prime_str, 2).unwrap() + number;
  }
  return number;
}

fn modpow(x: u64, mut y: u64, m: u64) -> u128 {
  let mut res: u128 = 1;     // Initialize result 
  
  // Update x if it is more 
  // than or equal to p 
  let mut x_new = (x % m) as u128; 
  let m_new = m as u128;
  if x == 0 {
    return 0;
  }

  while y > 0 {
    // If y is odd, multiply 
    // x with result 
    if (y & 1) == 1 {
      res = (res * x_new) % (m as u128);
    }
    // y must be even now 
    y = y >> 1;    // y = y/2 
    x_new = (x_new % m_new) * (x_new % m_new) % m_new;
  }  
  return res 
}
    
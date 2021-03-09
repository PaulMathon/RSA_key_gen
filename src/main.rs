mod gen_prime;
mod co_prime;

fn main() {
    let p = gen_prime::gen_prime() as u128;
    println!("p: {}", p);
    let q = gen_prime::gen_prime() as u128;
    println!("q: {}", q);
    let n = p * q;
    println!("n: {}", n);
    let phi = (p - 1) * (q - 1);
    println!("phi: {}", phi);
    let e = co_prime::find_co_prime(phi, 1000);
    println!("e: {}", e);
    let d = inverse(e as i128, phi as i128);
    println!("d: {}", d);
}

fn inverse(mut number: i128, mut modulus: i128) -> u128 {
    let m0 = modulus; 
    let mut y = 0;
    let mut x = 1;
  
    if modulus == 1 {
        return 0
    }
  
    while number > 1 {
        // q is quotient 
        let q = number / modulus;

        let mut t = modulus;

        // m is remainder now, process 
        // same as Euclid's algo 
        modulus = number % modulus;
        number = t;
        t = y;

        // Update x and y 
        y = x - q * y;
        x = t;

    }
  
    // Make x positive 
    if x < 0 {
        x = x + m0;

    }
  
    return x as u128;
}
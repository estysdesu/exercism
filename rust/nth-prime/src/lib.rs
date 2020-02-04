// nth gives the nth ascending prime from 0 to u32::max_value()
pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut i: u32 = 1;
    while primes.len() < (n + 1) as usize {
        i += 1;
        if is_prime(i) {
            primes.push(i)
        }
    }
    primes[n as usize]
}

// https://en.wikipedia.org/wiki/Primality_test
fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

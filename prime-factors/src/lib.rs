pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors:Vec<u64> = Vec::new();
    let mut n_num = n;
    let mut divisor = 2;

    while n_num > 1 {
        while n_num % divisor == 0 {
            prime_factors.push(divisor);
            n_num /= divisor;
        }
        divisor += 1;
    }

    prime_factors
}


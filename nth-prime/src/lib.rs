pub fn nth(n: u32) -> u32 {
   prime(n)
}


fn prime(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}

fn is_prime(n: u32) -> bool {
    let factors: Vec<_> =  (1..=n).filter(|x| n % x == 0).collect();
    if factors.len() == 2 {
        true
    } else {
        false
    }
}




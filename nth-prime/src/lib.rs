pub fn nth(n: u32) -> u32 {
   prime(n)
}


fn prime(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate as u32;
            }
            count += 1;
        }
        candidate += 1;
    }
}

/*
 * time consuming solution
 */
// fn is_prime(n: u32) -> bool {
//     let factors: Vec<_> =  (1..=n).filter(|x| n % x == 0).collect();
//     if factors.len() == 2 {
//         true
//     } else {
//         false
//     }
// }

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}




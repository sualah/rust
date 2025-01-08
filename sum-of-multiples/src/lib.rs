use std::collections::HashSet;
use std::ops::Range;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut values  = factors.iter();
    let mut unique_values: HashSet<u32> = HashSet::new();
    loop {
        match values.next() {
            Some(val) => {
                unique_values.extend(get_multiples(limit, *val))
            },
            None => break,
        }
    }
    unique_values.iter().sum()
}

pub fn get_multiples(limit: u32, num: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for i in 1..limit {
        if  i * num < limit {
            result.push(i * num);
        }
    }
    result
}
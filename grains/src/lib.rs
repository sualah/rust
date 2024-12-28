pub fn square(s: u32) -> u64 {
    match s {
        0  => panic!("Square must be positive"),
        1 => 1,
        2..65 => {
           let v = square(s - 1) * 2 ;
            v
        }
        _ => panic!()
    }
}

pub fn total() -> u64 {
   (1..=64).map(square).sum()
}

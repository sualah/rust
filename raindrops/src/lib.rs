pub fn raindrops(n: u32) -> String {
    let mut raindrop = String::new();
    if n % 3 ==0 {
        raindrop.push_str("Pling");
    }

    if n % 5 == 0 {
        raindrop.push_str("Plang");
    }

    if n % 7 == 0 {
        raindrop.push_str("Plong");
    }

    if (n % 7 != 0 ) && (n % 3 != 0) && ( n % 5 != 0) {
        raindrop = n.to_string();
    }

    raindrop
}

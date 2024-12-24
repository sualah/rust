pub fn is_armstrong_number(num: u32) -> bool {
    let  n_nums = num.to_string().chars().collect::<Vec<char>>();
    let count: u32 = n_nums.len() as u32;
    let mut total:u64 = 0;
    for n in n_nums {
        total += (n as u64 - '0' as u64).pow(count);
    }
    if num == 0 {
        return true;
    } else if total != num as u64 {
        return false;
    }
    true
}

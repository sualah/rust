pub fn verse(n: u32) -> String {
    
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.".to_string(),
        _ => {
            let prev_value = n - 1;
            let str = if prev_value == 1 { "bottle" } else { "bottles" };
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {prev_value} {str} of beer on the wall.").to_string()}
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result: String = "".to_string();
    for i in (end..=start).rev() {
        result += "\n\n";
        result += &verse(i).to_string();
    }
    result
}
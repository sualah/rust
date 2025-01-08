pub fn reply(message: &str) -> &str {
    let replies = ["Sure.", "Whoa, chill out!", "Calm down, I know what I'm doing!", "Fine. Be that way!",
    "Whatever."];
    let  num ;
    if message.trim().ends_with("?") && !is_uppercase(message) {
        num = 0 ;
    } else if  message.trim().ends_with("?") && is_uppercase(message) {
        num = 2 ;
    } else if message.trim().is_empty() {
        num = 3;
    }
    else if is_uppercase(message) {
        num = 1;
    }
    else {
        num = 4;
    }

    replies[num]
}

fn is_uppercase(word: &str) -> bool {
    // Filter out non-alphabetic characters
    let filtered_chars: Vec<char> = word.chars().filter(|&c| c.is_ascii_alphabetic()).collect();

    // Return false if no alphabetic characters are found
    if filtered_chars.is_empty() {
        return false;
    }

    // Check if all alphabetic characters are uppercase
    filtered_chars.iter().all(|&c| c.is_ascii_uppercase())
}

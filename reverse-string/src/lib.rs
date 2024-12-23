use unicode_reverse::reverse_grapheme_clusters_in_place;
pub fn reverse(input: &str) -> String {
   // input.chars().rev().collect()
    let mut rev_string = input.to_string();
    reverse_grapheme_clusters_in_place(&mut rev_string);
    rev_string
}

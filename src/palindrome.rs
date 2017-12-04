pub fn palindrome(string: &str) -> bool {
    for i in 0..string.len() / 2 {
        if string.chars().nth(i).expect("mid not found") !=
        string.chars().nth(string.len() - i - 1).expect("2nd not found") {
            return false
        }
    }
    true
}

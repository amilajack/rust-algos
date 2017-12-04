mod palindrome;

fn main() {
    assert_eq!(true, palindrome::palindrome("racecar"));
    assert_eq!(false, palindrome::palindrome("amila"));
}

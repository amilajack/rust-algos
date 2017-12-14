mod string;

fn main() {
    // String
    assert_eq!(true, string::palindrome("racecar"));
    assert_eq!(false, string::palindrome("amila"));
    assert_eq!(true, string::palindrome("awwa"));
    assert_eq!(true, string::palindrome("awa"));
}

mod string;
mod search;

fn main() {
    // String
    assert_eq!(true, string::palindrome("racecar"));
    assert_eq!(false, string::palindrome("amila"));
    assert_eq!(true, string::palindrome("awwa"));
    assert_eq!(true, string::palindrome("awa"));

    // Search
    assert_eq!(true, search::binary_search(vec![1, 2, 3], 2));
    assert_eq!(false, search::binary_search(vec![1, 2, 3], 12));
    assert_eq!(false, search::binary_search(vec![32, 56, 557, 1111], 12));
}

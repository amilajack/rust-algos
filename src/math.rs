/// Find the greatest commond divisor of two integers
/// a and b > 0. This is done in O(log(n)) time, where
/// n is the number of digits of the larger of a or b
pub fn euclidean_gcd(a: u32, b: u32) -> u32 {
    let (larger_number, smaller_number) = if a >= b {
        (a, b)
    } else {
        (b, a)
    };

    let remainder = larger_number % smaller_number;
    let k = ((larger_number / smaller_number) as f32).floor() as u32;

    if remainder == 0 {
        smaller_number
    } else {
        euclidean_gcd(k, remainder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_euclidea_gcd() {
        assert_eq!(euclidean_gcd(10, 5), 5);
        assert_eq!(euclidean_gcd(5, 15), 5);
        assert_eq!(euclidean_gcd(3, 15), 3);
        assert_eq!(euclidean_gcd(7, 11), 1);
    }
}

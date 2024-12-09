/// A simple atoi for numbers with less than 18 digits
pub fn atoi(s: &[u8]) -> i32 {
    let mut n: i64 = 0;
    for ch in s {
        let digit = ch - 48;
        if digit > 9 {
            return 0;
        }
        n = n * 10 + (digit as i64);
    }
    n as i32
}
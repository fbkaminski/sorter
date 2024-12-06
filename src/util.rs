pub fn fast_atoi(s: &[u8]) -> i32 {
    let mut n: i64 = 0;
    for ch in s {
        let digit = ch - 48;
        if digit > 9 {
            return 0;
        }
        n = n * 10 + (digit as i64);
    }
    return n as i32;
}

pub fn atoi(s: &[u8]) -> i32 {
    if s.len() < 19 {
        let mut sign: char = '+';
        let mut n;
        if s[0] == b'-' || s[0] == b'+' {
            if s[0] == b'-' {
                sign = '-';
            }
            n = fast_atoi(&s[1..s.len()])
        } else {
            n = fast_atoi(s);
        }

        if sign == '-' {
            n = -n;
        }

        // overflow
        if sign == '+' && n > i32::MAX {
            return i32::MAX;
        } else if sign == '-' && n < i32::MIN {
            return i32::MIN;
        }
        return n as i32;
    } // todo: slow atoi here
    return 0;
}

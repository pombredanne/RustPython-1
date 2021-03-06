use std::ops::{Bound, RangeBounds};

pub fn try_get_chars(s: &str, range: impl RangeBounds<usize>) -> Option<&str> {
    let mut chars = s.chars();
    let start = match range.start_bound() {
        Bound::Included(&i) => i,
        Bound::Excluded(&i) => i + 1,
        Bound::Unbounded => 0,
    };
    for _ in 0..start {
        chars.next()?;
    }
    let s = chars.as_str();
    let range_len = match range.end_bound() {
        Bound::Included(&i) => i + 1 - start,
        Bound::Excluded(&i) => i - start,
        Bound::Unbounded => return Some(s),
    };
    char_range_end(s, range_len).map(|end| &s[..end])
}

pub fn get_chars(s: &str, range: impl RangeBounds<usize>) -> &str {
    try_get_chars(s, range).unwrap()
}

#[inline]
pub fn char_range_end(s: &str, nchars: usize) -> Option<usize> {
    let i = match nchars.checked_sub(1) {
        Some(last_char_index) => {
            let (index, c) = s.char_indices().nth(last_char_index)?;
            index + c.len_utf8()
        }
        None => 0,
    };
    Some(i)
}

pub fn zfill(bytes: &[u8], width: usize) -> Vec<u8> {
    if width <= bytes.len() {
        bytes.to_vec()
    } else {
        let (sign, s) = match bytes.first() {
            Some(_sign @ b'+') | Some(_sign @ b'-') => {
                (unsafe { bytes.get_unchecked(..1) }, &bytes[1..])
            }
            _ => (&b""[..], bytes),
        };
        let mut filled = Vec::new();
        filled.extend_from_slice(sign);
        filled.extend(std::iter::repeat(b'0').take(width - bytes.len()));
        filled.extend_from_slice(s);
        filled
    }
}

/// Convert a string to ascii compatible, escaping unicodes into escape
/// sequences.
pub fn to_ascii(value: &str) -> String {
    let mut ascii = String::new();
    for c in value.chars() {
        if c.is_ascii() {
            ascii.push(c)
        } else {
            let c = c as i64;
            let hex = if c < 0x100 {
                format!("\\x{:02x}", c)
            } else if c < 0x10000 {
                format!("\\u{:04x}", c)
            } else {
                format!("\\U{:08x}", c)
            };
            ascii.push_str(&hex)
        }
    }
    ascii
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chars() {
        let s = "0123456789";
        assert_eq!(get_chars(s, 3..7), "3456");
        assert_eq!(get_chars(s, 3..7), &s[3..7]);

        let s = "0???????????? ?????????9";
        assert_eq!(get_chars(s, 3..7), "?????? ???");

        let s = "0????????????????????????????????9";
        assert_eq!(get_chars(s, 3..7), "????????????????");
    }
}

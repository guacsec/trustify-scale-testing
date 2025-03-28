use std::fmt::Display;

/// Truncate from the middle if the string is too long.
pub fn truncate_middle(s: impl Display, max_len: usize) -> String {
    let s = s.to_string();

    if s.len() <= max_len {
        s.to_string()
    } else {
        let keep = (max_len - 1) / 2; // Number of characters to keep from start and end
        let end_keep = max_len - keep - 1; // Ensure total length is exactly max_len
        format!("{}…{}", &s[..keep], &s[s.len() - end_keep..])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!("", truncate_middle("", 8));
        assert_eq!("0123456", truncate_middle("0123456", 8));
        assert_eq!("01234567", truncate_middle("01234567", 8));
        assert_eq!("012…5678", truncate_middle("012345678", 8),);
        assert_eq!("012…5678", truncate_middle("012345678012345678", 8));
    }
}

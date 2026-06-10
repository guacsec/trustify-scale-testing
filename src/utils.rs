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

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DisplayVec<T>(pub Vec<T>);

impl<T: std::fmt::Display> std::fmt::Display for DisplayVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strs: Vec<String> = self.0.iter().map(|item| item.to_string()).collect();
        write!(f, "{}", strs.join(","))
    }
}

/// This struct is used to store user-specific data that is needed for Goose transactions.
///
/// The advisory ID is randomly selected from the available advisories.
#[derive(Clone)]
pub struct GooseUserData {
    pub advisory_id: Option<String>,
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

    #[test]
    fn display_vec_single() {
        let v = DisplayVec(vec!["only"]);
        assert_eq!(v.to_string(), "only");
    }

    #[test]
    fn display_vec_empty() {
        let v: DisplayVec<String> = DisplayVec(vec![]);
        assert_eq!(v.to_string(), "");
    }

    #[test]
    fn display_vec_integers() {
        let v = DisplayVec(vec![1, 2, 3]);
        assert_eq!(v.to_string(), "1,2,3");
    }

    #[test]
    fn display_vec_serde_roundtrip() {
        let v: DisplayVec<String> = DisplayVec(vec!["x".into(), "y".into()]);
        let json = serde_json::to_string(&v).unwrap();
        let back: DisplayVec<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }
}

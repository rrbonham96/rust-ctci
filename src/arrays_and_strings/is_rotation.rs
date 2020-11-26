//! Assume you have a method isSubstring which checks if one word
//! is a substring of another. Given two strings s1 and s2, write
//! code to check if s2 is a rotation of s1 using only one call to 
//! isSubstring

pub fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() == s2.len() {
        let mut s2 = s2.to_string();
        s2.push_str(&s2.clone());
        return s2.contains(&s1);
    }
    false
}

#[cfg(test)]
mod test_is_rotation {
    use super::*;

    #[test]
    fn test_ok() {
        let s1 = "erbottlewat";
        let s2 = "waterbottle";
        assert!(is_rotation(s1, s2));
    }
}

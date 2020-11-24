//! Given two strings, write a method to decide if one is a permutation of the other

use std::collections::HashMap;

/// is_permutation returns true if one string is a permutation of the other
/// whitespace and case is included in this determination
pub fn is_permutation(s1: &String, s2: &String) -> bool {
    let mut s1_map = HashMap::new();
    let mut s2_map = HashMap::new();
    count_chars(&s1, &mut s1_map);  // O(s1)
    count_chars(&s2, &mut s2_map);  // O(s2)
    s1_map == s2_map
}

/// count_chars is used to keep a count of the occurences of each char in a string
/// this is O(n) for a string with length n
fn count_chars(s: &String, hm: &mut HashMap<char, u32>) {
    for char in s.chars() {
        let count = hm.entry(char).or_insert(0);
        *count += 1;
    }
}

#[cfg(test)]
mod test_is_permutation {
    use super::*;

    #[test]
    fn test_is_permutation() {
        let base = "abc ".to_string();
        let perms = vec![
            "a bc",
            "ab c",
            "b ca",
            " abc",
        ];

        for perm in perms.iter().map(|&s| s.to_string()) {
            assert!(is_permutation(&perm, &base));
        }
    }

    #[test]
    fn test_is_not_permutation() {
        let base = "abc ".to_string();
        let perms = vec![
            "a bc ",
            "ab cd",
            "b fca",
            "x abc",
        ];

        for perm in perms.iter().map(|&s| s.to_string()) {
            assert!(!is_permutation(&perm, &base));
        }
    }
}
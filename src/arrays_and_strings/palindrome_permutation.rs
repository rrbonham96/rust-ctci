//! Given a string, write a function to check if it is a permutation of
//! a palindrome. A palindrome is a word or phrase that is the same forwards
//! and backwards. A permutation is a rearangement of letters.
//! The aplindrome does not need to be limited to just dictionary words.
//! You an ignore casing and non-letter characters

use std::collections::HashMap;

pub fn is_palindrome_permuation(s: &String) -> bool {
    let s= s.to_lowercase().trim().to_string();
    let s = remove_inner_whitespace(&s);
    let char_count = count_chars(&s);
    let mut odd_count = 0;
    for val in char_count.values() {
        if val % 2 != 0 {
            odd_count += 1;
            if odd_count > 2 {
                return false;
            }
        }
    }
    true
}

fn remove_inner_whitespace(s: &String) -> String {
    s.split_ascii_whitespace().collect::<Vec<_>>().join("")
}

fn count_chars(s: &String) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    char_count
}

#[cfg(test)]
mod test_palindrome {
    use super::*;

    #[test]
    fn test_ok() {
        let palindromes = vec![
            "oko ko",
            "racecar",
            "  a b a  ",
        ];

        for p in palindromes.iter().map(|s| s.to_string()) {
            assert!(is_palindrome_permuation(&p));
        }
    }
}
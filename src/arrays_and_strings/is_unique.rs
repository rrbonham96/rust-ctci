//! Implement an algorithm to determine if a string has all unique characters.
//! What if you cannot use additional data structures?

use std::collections::HashMap;

/// is_unique with hashmap will determine if the string only contains unique characters
/// by keeping a count for every character in a hashmap, and returning false if the count
/// for any character is greater than 1
pub fn is_unique_with_hashmap(string: &String) -> bool {
    let mut char_count = HashMap::new();
    for char in string.chars() {
        let count = char_count.entry(char).or_insert(0);
        *count += 1;
        if *count > 1 {
            return false;
        }
    }
    true
}

/// is_unique is similar to the above, but does not use a hashmap
/// instead, it uses a bit vector/array to reduce space complexity
pub fn is_unique(string: &String) -> bool {
    let mut bit_checker = 0u128;
    // Iterate over characters as their ASCII value
    for char in string.chars().map(|c| c as u32) {
        if bit_checker & (1 << char) > 0 {
            return false
        }
        bit_checker |= 1 << char;
    }
    true
}

/// is_unique_slow does not use another data structure
/// instead, it searches the list for every character to check for duplicates
pub fn is_unique_slow(string: &String) -> bool {
    for (i, my_char) in string.chars().enumerate() {
        for cmp_char in string.chars().skip(i + 1) {
            if my_char == cmp_char {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test_is_unique {
    use super::*;

    #[test]
    fn test_unique() {
        let unique_strings = vec![
            "",
            "abc",
            "def",
            "xyz12345679",
        ];
        for string in unique_strings.iter().map(|&x| x.to_string()) {
            assert!(is_unique_with_hashmap(&string));
            assert!(is_unique(&string));
            assert!(is_unique_slow(&string))
        }
    }

    #[test]
    fn test_not_unique() {
        let unique_strings = vec![
            "abca",
            "defe",
            "xyz123451679",
        ];
        for string in unique_strings.iter().map(|&x| x.to_string()) {
            assert!(!is_unique_with_hashmap(&string));
            assert!(!is_unique(&string));
            assert!(!is_unique_slow(&string))
        }
    }
}
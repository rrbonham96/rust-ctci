//! There are three types of edits that can be performed on strings:
//! insert a character, remove a character, or replace a character.
//! Given two strings, write a function to check if they are one edit 
//! (or zero edits) away.

pub fn is_one_away(s1: &str, s2: &str) -> bool {
    let char_count_diff = s1.chars().count() as isize - s2.chars().count() as isize;
    match char_count_diff {
        0 => is_replacement(&s1, &s2),
        1 => is_insertion(&s1, &s2),
        -1 => is_insertion(&s2, &s1),
        _ => false
    }
}

fn is_replacement(s1: &str, s2: &str) -> bool {
    let mut char_diff = false;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if char_diff {
                return false;
            }

            char_diff = true;
        }
    }
    true
}

fn is_insertion(long: &str, short: &str) -> bool {
    let mut char_diff = false;
    let mut long_chars = long.chars();
    let mut short_chars = short.chars();
    let mut e1 = long_chars.next();
    let mut e2 = short_chars.next();
    
    while let (Some(c1), Some(c2)) = (e1, e2) {
        if c1 != c2 {
            if char_diff {
                return false;
            }
            char_diff = true;
            long_chars.next();
        }

        e1 = short_chars.next();
        e2 = long_chars.next();
    }
    true
}

#[cfg(test)]
mod test_one_away {
    use super::*;

    #[test]
    fn test_ok() {
        assert!(is_one_away("dog", "dag"));
        assert!(is_one_away("dog", "dogs"));
        assert!(is_one_away("dogg", "dog"));
        assert!(is_one_away("og", "dog"));
        assert!(is_one_away("dog ", "dog"));
    }
}
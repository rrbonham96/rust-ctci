//! Implement a method to perform basic string compression using the
//! counts of repeated characters. For example, the string
//! aabcccccaaa would become a2b1c5a3. If the compressed string
//! would not become smaller than the original string, your method
//! should return the original string. You can assume the string has
//! only uppercase and lowercase letters

pub fn compress(s: &str) -> String {
    let mut comp: Vec<String> = vec![];
    let mut count = 0;
    let mut char_iter = s.chars().peekable();

    while let Some(c) = char_iter.next() {
        count += 1;
        match char_iter.peek() {
            Some(&next_c) => {
                if next_c != c {
                    comp.push(format!("{}{}", c, count).to_string());
                    count = 0;
                }
            },
            None => comp.push(format!("{}{}", c, count).to_string()),
        }
    }

    let comp_str = comp.join("");
    if comp_str.len() < s.len() { comp_str } else { String::from(s)}
}

#[cfg(test)]
mod test_compression {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
    }
}
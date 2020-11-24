//! Write a method to replace all spaces in a string with '%20'
//! Assume that the string has sufficient space at the end to
//! hold the additional characters, and that you are given the 'true'
//! length of the string

/// urlify changes a string in place so spaces are replaced with '%20'
/// the string must have sufficient space at the end for extra characters
/// the string is represented as a char slice, since rust strings aren't char arrays
/// assumes that there are no intentional spaces at the end of the array
pub fn urlify(string: &mut[char], length: usize) {
    let space_count = count_spaces(&string[..], length);
    let mut new_index = length - 1 + space_count * 2;

    for old_index in (0..length).rev() {
        if string[old_index] == ' ' {
            string[new_index] = '0';
            string[new_index - 1] = '2';
            string[new_index - 2] = '%';
            new_index -= 3;
        } else {
            string[new_index] = string[old_index];
            // This is to prevent underflow (panic) on the last iteration
            if new_index > 0 {
                new_index -= 1;
            }
        }
    }
}

/// count the number of spaces from the true length of the string
fn count_spaces(string: &[char], length: usize) -> usize {
    let mut count = 0usize;
    for i in 0..length {
        if string[i] == ' ' {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test_urlify {
    use super::*;

    #[test]
    fn test_urlify() {
        let string = "Mr John Smith    ";
        let mut as_chars: Vec<_> = string.chars().collect();

        urlify(&mut as_chars[..], 13);

        let as_string: String = as_chars.iter().collect();
        assert_eq!(as_string, "Mr%20John%20Smith");
    }
}
//! Given an image represented by an N x N matrix,
//! where each pixel in the image is represented by an integer,
//! write a method to rotate the image by 90 degrees. Can you
//! do this in place?

type Matrix = Vec<Vec<u32>>;

pub fn rotate(m: &mut Matrix) -> Result<(), &'static str> {
    if m.len() == 0 || m.len() != m[0].len() {
        return Err("Provided matrix is not N x N");
    }

    let n = m.len();
    let layers = n / 2;

    for layer in 1..=layers {
        for i in (layer - 1)..(n - layer) {
            let old_top = m[layer - 1][i];
            m[layer - 1][i] = m[n - i - 1][layer - 1];
            m[n - i - 1][layer - 1] = m[n - layer][n - i - 1];
            m[n - layer][n - i - 1] = m[i][n - layer];
            m[i][n - layer] = old_top;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_rotation {
    use super::*;

    #[test]
    fn test_ok() {
        let mut matrix: Matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected: Matrix = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ];
        rotate(&mut matrix).unwrap();
        assert_eq!(matrix, expected);

    }
}

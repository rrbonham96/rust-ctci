//! Write an algorithm such that if an element in an
//! M x N matrix is 0 it's entire row and column
//! are set to 0

type Matrix = Vec<Vec<u32>>;

pub fn zero_matrix(m: &mut Matrix) {
    let mut rows = vec![false; m.len()];
    let mut cols = vec![false; m[0].len()];

    for (i, row) in m.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                rows[i] = true;
                cols[j] = true;
            }
        }
    }

    for i in 0..rows.len() {
        if rows[i] {
            zero_row(m, i);
        }
    }
        
    for i in 0..cols.len() {
        if cols[i] {
            zero_col(m, i);
        }
    }
}

fn zero_row(m: &mut Matrix, row: usize) {
    for i in 0..m[row].len() {
        m[row][i] = 0;
    }
}

fn zero_col(m: &mut Matrix, col: usize) {
    for i in 0..m.len() {
        m[i][col] = 0;
    }
}

#[cfg(test)]
mod test_zero_matrix {
    use super::*;

    #[test]
    fn test_ok() {
        let mut matrix: Matrix = vec![
            vec![0, 1, 0],
            vec![2, 3, 4],
        ];
        let expected: Matrix = vec![
            vec![0, 0, 0],
            vec![0, 3, 0],
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }
}

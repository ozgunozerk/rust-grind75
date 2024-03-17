impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (mat.len(), mat[0].len());
        let mut res = vec![vec![0; cols]; rows] as Vec<Vec<i32>>;

        let get_cell = |res: &Vec<Vec<i32>>, row, col| {
            *res.get(row)
                .unwrap_or(&vec![])
                .get(col)
                .unwrap_or(&(rows as i32 + cols as i32))
        };

        // start from TOP-LEFT
        for row in 0..rows {
            for col in 0..cols {
                res[row][col] = mat[row][col]
                    * (1 + get_cell(&res, row, col - 1).min(get_cell(&res, row - 1, col)));
            }
        }

        // start from BOTTOM-RIGHT
        for row in (0..rows).rev() {
            for col in (0..cols).rev() {
                res[row][col] = mat[row][col]
                    * res[row][col]
                        .min(1 + get_cell(&res, row, col + 1).min(get_cell(&res, row + 1, col)));
            }
        }

        res
    }
}

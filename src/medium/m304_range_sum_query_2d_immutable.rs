struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    prefix_sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut prefix_sums = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                prefix_sums[i + 1][j + 1] =
                    matrix[i][j] + prefix_sums[i + 1][j] + prefix_sums[i][j + 1]
                        - prefix_sums[i][j];
            }
        }

        NumMatrix {
            matrix,
            prefix_sums,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.prefix_sums[row2 as usize + 1][col2 as usize + 1]
            - self.prefix_sums[row1 as usize][col2 as usize + 1]
            - self.prefix_sums[row2 as usize + 1][col1 as usize]
            + self.prefix_sums[row1 as usize][col1 as usize]
    }
}

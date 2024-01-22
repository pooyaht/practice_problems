struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn check_neighbors(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
            grid[row as usize][col as usize] = '0';
            for (row, col) in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .filter_map(|(r, c)| {
                    let row = *r + row;
                    let col = *c + col;
                    if row >= 0 && row < grid.len() as i32 && col >= 0 && col < grid[0].len() as i32
                    {
                        Some((row, col))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
            {
                if grid[row as usize][col as usize] == '1' {
                    check_neighbors(grid, row, col);
                }
            }
        }
        let mut grid = grid;
        let mut islands_count = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '1' {
                    islands_count += 1;
                    check_neighbors(&mut grid, row as i32, col as i32);
                }
            }
        }
        islands_count
    }
}

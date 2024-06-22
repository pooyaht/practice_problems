struct Solution;
impl Solution {
    fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];

        for str_ in strs.into_iter() {
            let zeros = str_.chars().filter(|c| c == &'0').count();
            let ones = str_.len() - zeros;
            for i in (zeros..=m as usize).rev() {
                for j in (ones..=n as usize).rev() {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i - zeros][j - ones] + 1);
                }
            }
        }

        dp[m as usize][n as usize]
    }
}

struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let total: i32 = stones.iter().sum();
        let target = total / 2;
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;

        for &stone in &stones {
            for i in (stone..=target).rev() {
                dp[i as usize] |= dp[(i - stone) as usize];
            }
        }

        for i in (0..=target).rev() {
            if dp[i as usize] {
                return total - 2 * i;
            }
        }

        0
    }
}

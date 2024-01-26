struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut sum = 0;
        let mut ans = std::i32::MAX;
        for r in 0..nums.len() {
            sum += nums[r];
            while sum >= target {
                ans = ans.min((r - l + 1) as i32);
                sum -= nums[l];
                l += 1;
            }
        }
        if ans == std::i32::MAX {
            0
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}

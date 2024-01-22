//Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
pub struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::new();
        if nums.len() < 3 {
            return res;
        }
        if nums[0] > 0 || nums[nums.len() - 1] < 0 {
            return res;
        }
        for i in 0..nums.len() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    right -= 1;
                    left += 1;
                }
                if sum < 0 {
                    left += 1;
                }
                if sum > 0 {
                    right -= 1;
                }
            }
        }
        res.dedup();
        res
    }
}

pub struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        nums.iter()
            .fold((1, 1, i32::MIN), |acc, val| {
                let (curr_min, curr_max, mut result) = acc;
                let val = *val;
                let temp_min = i32::checked_mul(curr_min, val).unwrap_or(1);
                let temp_max = i32::checked_mul(curr_max, val).unwrap_or(1);
                let max = val.max(temp_max).max(temp_min);
                let min = val.min(temp_min).min(temp_max);
                result = result.max(max);
                (min, max, result)
            })
            .2
    }
}

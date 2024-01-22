struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums[1..]
            .iter()
            .fold((nums[0], nums[0]), |acc, &x| {
                let (sum, max) = acc;
                (x.max(sum + x), max.max(x.max(sum + x)))
            })
            .1
    }
}

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            let m = (l + r) / 2;
            match nums[m].cmp(&target) {
                std::cmp::Ordering::Less => {
                    l = m + 1;
                }
                std::cmp::Ordering::Greater => {
                    r = m - 1;
                }
                std::cmp::Ordering::Equal => {
                    return m as i32;
                }
            }
        }
        -1
    }
}

struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0_usize, nums.len() - 1);
        while start < end {
            let middle = (start + end) / 2;
            if nums[middle] > nums[middle + 1] {
                return nums[middle + 1];
            } else if nums[middle] > nums[end] {
                start = middle;
            } else {
                end = middle;
            }
        }
        nums[0]
    }
}

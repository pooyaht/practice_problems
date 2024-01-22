pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn _search(nums: &[i32], target: i32, start: usize, end: usize) -> i32 {
            let middle_index = (start + end) / 2;
            if nums[middle_index] == target {
                return middle_index as i32;
            }
            if end <= start {
                return -1;
            }

            if nums[middle_index] >= nums[start] {
                if target < nums[middle_index] && target >= nums[start] {
                    return _search(nums, target, start, middle_index);
                }
                _search(nums, target, middle_index + 1, end)
            } else {
                if target > nums[end] || target < nums[middle_index] {
                    return _search(nums, target, start, middle_index);
                }
                _search(nums, target, middle_index + 1, end)
            }
        }
        _search(&nums, target, 0, nums.len() - 1)
    }
}

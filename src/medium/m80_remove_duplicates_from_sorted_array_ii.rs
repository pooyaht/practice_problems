struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }
        let mut i = 2;
        for j in 2..nums.len() {
            if nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        let expected = vec![1, 1, 2, 2, 3];
        for i in 0..5 {
            assert_eq!(nums[i], expected[i]);
        }
    }
}

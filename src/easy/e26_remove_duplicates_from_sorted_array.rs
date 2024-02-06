struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        (i + 1) as i32
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_with_duplicates() {
        let mut nums = vec![1, 1, 2];
        let expected_nums = vec![1, 2];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            expected_nums.len() as i32
        );
        for i in 0..expected_nums.len() {
            assert_eq!(nums[i], expected_nums[i]);
        }
    }
    #[test]
    fn test_without_duplicates() {
        let mut nums = vec![1, 2, 3];
        let expected_nums = vec![1, 2, 3];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            expected_nums.len() as i32
        );
        for i in 0..expected_nums.len() {
            assert_eq!(nums[i], expected_nums[i]);
        }
    }
}

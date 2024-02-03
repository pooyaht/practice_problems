struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            let sum = numbers[l] + numbers[r];
            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
                std::cmp::Ordering::Greater => {
                    r -= 1;
                }
                std::cmp::Ordering::Less => {
                    l += 1;
                }
            }
        }
        Vec::new()
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![1, 4], 3), vec![]);
    }
}

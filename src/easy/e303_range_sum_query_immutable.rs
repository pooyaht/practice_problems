struct NumArray {
    prefix_sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let prefix_sums: Vec<_> = nums
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect();
        NumArray { prefix_sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            return self.prefix_sums[right as usize];
        }
        self.prefix_sums[right as usize] - self.prefix_sums[(left - 1) as usize]
    }
}

mod tests {
    use super::*;
    #[test]
    fn test() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new(nums);
        assert_eq!(obj.sum_range(0, 2), 1);
        assert_eq!(obj.sum_range(2, 5), -1);
        assert_eq!(obj.sum_range(0, 5), -3);
    }
}

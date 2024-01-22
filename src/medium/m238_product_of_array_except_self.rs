struct Solution;
impl Solution {
    fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![1; nums.len()];
        let (mut pre, mut suf) = (1, 1);

        for i in 0..nums.len() {
            ans[i] *= pre;
            pre *= nums[i];
            ans[nums.len() - 1 - i] *= suf;
            suf *= nums[nums.len() - 1 - i];
        }

        ans
    }
}

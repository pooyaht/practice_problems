struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut l = 0;
        let mut sum = 0;
        let mut ans = 0;
        for r in 0..arr.len() {
            sum += arr[r];
            if r - l + 1 > k as usize {
                sum -= arr[l];
                l += 1;
            }
            if r - l + 1 == k as usize && sum.div_euclid(k) >= threshold {
                ans += 1;
            }
        }
        ans
    }
}

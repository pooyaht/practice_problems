struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        map.insert(0, 1);
        for n in nums {
            sum += n;
            if let Some(v) = map.get(&(sum - k)) {
                ans += v;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        ans
    }
}

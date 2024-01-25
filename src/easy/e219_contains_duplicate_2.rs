struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            if seen.get(&x).map_or(false, |&j| (i as i32 - j).abs() <= k) {
                return true;
            }
            seen.insert(x, i as i32);
        }
        false
    }
}

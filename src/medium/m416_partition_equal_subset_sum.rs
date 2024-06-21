use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum = nums.iter().sum::<i32>();
        if total_sum % 2 != 0 {
            return false;
        }
        let mut cache = HashSet::new();
        cache.insert(0);

        let target = total_sum / 2;
        for &num in nums.iter() {
            let mut new_cache = HashSet::new();
            for &sum in cache.iter() {
                if sum + num == target {
                    return true;
                }
                new_cache.insert(sum + num);
                new_cache.insert(sum);
            }
            cache = new_cache;
        }
        cache.contains(&target)
    }
}

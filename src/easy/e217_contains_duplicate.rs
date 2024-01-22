use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<i32> = std::collections::HashSet::from_iter(nums.clone());
        set.len() < nums.len()
    }
}

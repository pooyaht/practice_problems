struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
        nums.iter().for_each(|val| {
            *map.entry(*val).or_insert(0) += 1;
        });
        let mut vec = map.iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.1.cmp(a.1));
        vec.iter().take(k as usize).map(|x| *x.0).collect()
    }
}

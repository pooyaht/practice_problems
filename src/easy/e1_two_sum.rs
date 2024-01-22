use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let mut result = Vec::<i32>::new();
        for (index, &num) in nums.iter().enumerate() {
            if let Some(&second_index) = hashmap.get(&(target - num)) {
                result.push(second_index as i32);
                result.push(index as i32);
                break;
            }
            hashmap.insert(num, index);
        }
        result
    }
}

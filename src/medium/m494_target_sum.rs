struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        pub fn find_target_sum_ways_util(
            nums: &Vec<i32>,
            target: i32,
            total_sum: i32,
            index: usize,
            sum: i32,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if index == nums.len() {
                if sum == target {
                    return 1;
                } else {
                    return 0;
                }
            };
            if cache[index][(sum + total_sum) as usize] != 0 {
                return cache[index][(sum + total_sum) as usize];
            }

            let count = find_target_sum_ways_util(
                nums,
                target,
                total_sum,
                index + 1,
                sum + nums[index],
                cache,
            ) + find_target_sum_ways_util(
                nums,
                target,
                total_sum,
                index + 1,
                sum - nums[index],
                cache,
            );
            cache[index][(sum + total_sum) as usize] = count;
            count
        }

        let total_sum = nums.iter().sum::<i32>();
        if total_sum < target || (total_sum - target) % 2 != 0 {
            return 0;
        }
        let mut cache = vec![vec![0; (2 * total_sum + 1) as usize]; 20];
        find_target_sum_ways_util(&nums, target, total_sum, 0, 0, cache.as_mut())
    }
}

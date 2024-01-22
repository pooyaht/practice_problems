struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(mut nums: Vec<i32>) -> i32 {
        let mut total_sum = nums[0];
        let (_, global_min, _, global_max) =
            nums.iter()
                .skip(1)
                .fold((nums[0], nums[0], nums[0], nums[0]), |acc, val| {
                    let (mut current_min, mut global_min, mut current_max, mut global_max) = acc;
                    current_min = current_min.min(0);
                    current_max = current_max.max(0);
                    current_max += val;
                    current_min += val;
                    global_min = global_min.min(current_min);
                    global_max = global_max.max(current_max);
                    total_sum += val;
                    (current_min, global_min, current_max, global_max)
                });
        if global_max.is_negative() {
            global_max
        } else {
            (total_sum - global_min).max(global_max)
        }
    }
}

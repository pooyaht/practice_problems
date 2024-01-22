struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|interval| (interval[0], interval[1]));
        intervals[1..]
            .iter()
            .fold((0, &intervals[0]), |(remove_count, ptr), interval| {
                if ptr[1] <= interval[0] {
                    (remove_count, interval)
                } else if interval[0] >= ptr[0] && interval[1] <= ptr[1] {
                    (remove_count + 1, interval)
                } else {
                    (remove_count + 1, ptr)
                }
            })
            .0
    }
}

struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| v[0]);
        intervals.iter().fold(Vec::new(), |mut acc, interval| {
            if acc.is_empty() || acc.last().unwrap()[1] < interval[0] {
                acc.push(interval.clone());
            } else {
                acc.last_mut().unwrap()[1] = acc.last().unwrap()[1].max(interval[1]);
            }
            acc
        })
    }
}

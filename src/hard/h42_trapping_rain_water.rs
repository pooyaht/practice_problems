struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_left_side = vec![0; height.len()];
        let mut max_right_side = vec![0; height.len()];
        for i in 1..height.len() - 1 {
            max_left_side[i] = max_left_side[i - 1].max(height[i - 1]);
        }
        for i in (1..height.len() - 1).rev() {
            max_right_side[i] = max_right_side[i + 1].max(height[i + 1]);
        }
        println!("{:?}, {:?}", max_left_side, max_right_side);
        let mut res = 0;
        for i in 1..height.len() - 1 {
            res += (max_left_side[i].min(max_right_side[i]) - height[i]).max(0);
        }
        res
    }
}

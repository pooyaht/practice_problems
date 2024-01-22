struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let height_array = height;
        let (mut left_side_index, mut right_side_index) = (0, height_array.len() - 1);
        let mut max_area = 0;
        while left_side_index < right_side_index {
            let width = (right_side_index - left_side_index) as i32;
            let height = height_array[left_side_index].min(height_array[right_side_index]);
            max_area = max_area.max(width * height);
            if height_array[left_side_index] < height_array[right_side_index] {
                left_side_index += 1;
            } else {
                right_side_index -= 1;
            }
        }
        max_area
    }
}

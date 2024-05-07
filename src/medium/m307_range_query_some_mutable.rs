#[derive(Default, Debug, Clone)]
struct SegmentTree {
    left_node: Option<Box<SegmentTree>>,
    right_node: Option<Box<SegmentTree>>,
    lower_range: i32,
    upper_range: i32,
    value: i32,
}

impl SegmentTree {
    fn build(nums: &[i32], lower_range: i32, upper_range: i32) -> SegmentTree {
        if lower_range == upper_range {
            SegmentTree {
                value: nums[lower_range as usize],
                lower_range,
                upper_range,
                left_node: None,
                right_node: None,
            }
        } else {
            let middle_range = (lower_range + upper_range) / 2;
            let mut root = SegmentTree {
                value: 0,
                lower_range,
                upper_range,
                left_node: None,
                right_node: None,
            };
            root.left_node = Some(Box::new(SegmentTree::build(
                nums,
                lower_range,
                middle_range,
            )));
            root.right_node = Some(Box::new(SegmentTree::build(
                nums,
                middle_range + 1,
                upper_range,
            )));
            root.value = root
                .right_node
                .as_ref()
                .unwrap_or(&Box::<SegmentTree>::default())
                .value
                + root
                    .left_node
                    .as_ref()
                    .unwrap_or(&Box::<SegmentTree>::default())
                    .value;
            root.lower_range = lower_range;
            root.upper_range = upper_range;
            root
        }
    }

    fn update(&mut self, index: i32, value: i32) {
        if self.lower_range == self.upper_range {
            self.value = value;
            return;
        }

        let middle_range = (self.lower_range + self.upper_range) / 2;
        if index <= middle_range {
            self.left_node.as_mut().unwrap().update(index, value);
        } else {
            self.right_node.as_mut().unwrap().update(index, value);
        }
        self.value =
            self.right_node.as_ref().unwrap().value + self.left_node.as_ref().unwrap().value
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == self.lower_range && right == self.upper_range {
            return self.value;
        }

        let middle_range = (self.lower_range + self.upper_range) / 2;
        if left > middle_range && self.right_node.is_some() {
            self.right_node.as_ref().unwrap().sum_range(left, right)
        } else if right <= middle_range && self.left_node.is_some() {
            self.left_node.as_ref().unwrap().sum_range(left, right)
        } else {
            let (mut right_val, mut left_val) = (0, 0);
            if let Some(left_node) = self.left_node.as_ref() {
                left_val = left_node.sum_range(left, middle_range);
            }
            if let Some(right_node) = self.right_node.as_ref() {
                right_val = right_node.sum_range(middle_range + 1, right);
            }
            left_val + right_val
        }
    }
}

pub struct NumArray {
    segment_tree: SegmentTree,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let st = SegmentTree::build(nums.as_slice(), 0, (nums.len() - 1) as i32);
        Self { segment_tree: st }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        self.segment_tree.update(index, val);
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.segment_tree.sum_range(left, right)
    }
}

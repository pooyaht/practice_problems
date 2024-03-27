use std::collections::HashMap;

struct UnionFind {
    rank: [i32; 100001],
    parent: [i32; 100001],
    count: [i32; 100001],
}

impl UnionFind {
    fn New() -> Self {
        UnionFind {
            rank: [0; 100001],
            parent: (0..100001).collect::<Vec<i32>>().try_into().unwrap(),
            count: [1; 100001],
        }
    }

    fn find(&mut self, mut node_id: i32) -> i32 {
        let mut parent_node_id = self.parent[node_id as usize];
        while parent_node_id != node_id {
            self.parent[node_id as usize] = self.parent[parent_node_id as usize];
            node_id = parent_node_id;
            parent_node_id = self.parent[node_id as usize];
        }
        node_id
    }

    fn union(&mut self, node1_id: i32, node2_id: i32) -> bool {
        let (node1_parent_id, node2_parent_id) = (self.find(node1_id), self.find(node2_id));
        if node1_parent_id == node2_parent_id {
            return false;
        }

        match self.rank[node1_parent_id as usize].cmp(&self.rank[node2_parent_id as usize]) {
            std::cmp::Ordering::Less => {
                self.parent[node1_parent_id as usize] = node2_parent_id;
                self.count[node2_parent_id as usize] += self.count[node1_parent_id as usize];
            }
            std::cmp::Ordering::Greater => {
                self.parent[node2_parent_id as usize] = node1_parent_id;
                self.count[node1_parent_id as usize] += self.count[node2_parent_id as usize];
            }
            std::cmp::Ordering::Equal => {
                self.parent[node2_parent_id as usize] = node1_parent_id;
                self.rank[node1_parent_id as usize] += 1;
                self.count[node1_parent_id as usize] += self.count[node2_parent_id as usize];
            }
        }

        true
    }
}

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut uf = UnionFind::New();
        let mut num_to_index = HashMap::<i32, i32>::with_capacity(2000);
        let mut idx = 0;
        for (i, num) in nums.iter().enumerate() {
            num_to_index.entry(*num).or_insert_with(|| {
                idx += 1;
                idx
            });
        }

        for (num, index) in num_to_index.iter() {
            if let Some(previous_index) = num_to_index.get(&(*num - 1)) {
                uf.union(*index, *previous_index);
            }
        }

        *uf.count.iter().max().unwrap_or(&0)
    }
}

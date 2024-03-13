struct UnionFind {
    rank: [i32; 1001],
    parent: [i32; 1001],
}

impl UnionFind {
    fn New() -> Self {
        UnionFind {
            rank: (0..1001).collect::<Vec<i32>>().try_into().unwrap(),
            parent: (0..1001).collect::<Vec<i32>>().try_into().unwrap(),
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
            }
            std::cmp::Ordering::Greater => {
                self.parent[node2_parent_id as usize] = node1_parent_id;
            }
            std::cmp::Ordering::Equal => {
                self.parent[node2_parent_id as usize] = node1_parent_id;
                self.rank[node1_parent_id as usize] += 1;
            }
        }

        true
    }
}

struct Solution;
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        let mut uf = UnionFind::New();
        for edge in edges {
            if !uf.union(edge[0], edge[1]) {
                ans = edge;
            }
        }
        ans
    }
}

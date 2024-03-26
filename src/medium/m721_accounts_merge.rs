use std::collections::HashMap;

struct UnionFind {
    rank: [i32; 1001],
    parent: [i32; 1001],
}

impl UnionFind {
    fn New() -> Self {
        UnionFind {
            rank: [0; 1001],
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
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut uf = UnionFind::New();
        let mut email_to_index = HashMap::<&str, i32>::with_capacity(1000);
        for (i, user) in accounts.iter().enumerate() {
            for email in user.iter().skip(1) {
                email_to_index
                    .entry(email)
                    .and_modify(|j: &mut i32| {
                        uf.union(i as i32, *j);
                    })
                    .or_insert(i as i32);
            }
        }
        let mut emails_per_userindex = (0..accounts.len() + 1)
            .map(|_| Vec::new())
            .collect::<Vec<Vec<String>>>();

        for (email, i) in email_to_index.into_iter() {
            let parent_id = uf.find(i);
            emails_per_userindex[parent_id as usize].push(email.to_owned())
        }

        let mut result = Vec::with_capacity(accounts.len());
        for (i, info) in accounts.iter().enumerate() {
            if emails_per_userindex[i].is_empty() {
                continue;
            }
            let mut user = Vec::new();
            user.push(info[0].to_owned());
            emails_per_userindex[i].sort();
            user.append(emails_per_userindex[i].as_mut());
            result.push(user);
        }
        result
    }
}

use std::collections::HashSet;

struct Trie {
    root: TrieNode,
}

struct TrieNode {
    is_word: bool,
    children: std::collections::HashMap<u8, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            is_word: false,
            children: std::collections::HashMap::with_capacity(26),
        }
    }
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            node = node
                .children
                .entry(*c)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.as_bytes() {
            if !node.children.contains_key(c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }
        node.is_word
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.as_bytes() {
            if !node.children.contains_key(&c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }
        true
    }
}

struct Solution;
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        let mut words = words;
        words.dedup();
        words.iter().for_each(|word| {
            trie.insert(word);
        });

        let board: Vec<Vec<u8>> = board
            .into_iter()
            .map(|row| row.into_iter().map(|c| c as u8).collect())
            .collect();
        let mut result = HashSet::<String>::new();
        let mut visited = HashSet::<(usize, usize)>::new();
        'first_loop: for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::dfs(
                    &board,
                    &mut trie.root,
                    &mut result,
                    &mut visited,
                    i,
                    j,
                    &mut Vec::new(),
                );
                if result.len() == words.len() {
                    break 'first_loop;
                }
            }
        }
        result.into_iter().collect()
    }

    fn dfs(
        board: &Vec<Vec<u8>>,
        trie: &mut TrieNode,
        result: &mut HashSet<String>,
        visited: &mut HashSet<(usize, usize)>,
        i: usize,
        j: usize,
        path: &mut Vec<char>,
    ) {
        if i >= board.len()
            || j >= board[0].len()
            || visited.contains(&(i, j))
            || !trie.children.contains_key(&board[i][j])
        {
            return;
        }
        visited.insert((i, j));

        let c = board[i][j];
        path.push(c as char);
        let node = trie.children.get_mut(&c).unwrap();
        if node.is_word {
            result.insert(path.iter().collect::<String>());
            node.is_word = false;
        }

        Self::dfs(board, node, result, visited, i - 1, j, path);
        Self::dfs(board, node, result, visited, i, j - 1, path);
        Self::dfs(board, node, result, visited, i + 1, j, path);
        Self::dfs(board, node, result, visited, i, j + 1, path);

        if node.children.is_empty() {
            trie.children.remove(&c);
        }
        visited.remove(&(i, j));
        path.pop();
    }
}

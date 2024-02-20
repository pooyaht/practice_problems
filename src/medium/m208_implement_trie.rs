struct Trie {
    root: TrieNode,
}

struct TrieNode {
    is_word: bool,
    children: std::collections::HashMap<char, TrieNode>,
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

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if !node.children.contains_key(&c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }
        node.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if !node.children.contains_key(&c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }
        true
    }
}

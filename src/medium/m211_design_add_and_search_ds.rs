struct WordDictionary {
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
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        word.chars()
            .fold(&mut self.root, |node: &mut TrieNode, c| {
                node.children.entry(c).or_insert(TrieNode::new())
            })
            .is_word = true;
    }

    fn search(&self, word: String) -> bool {
        self._search(&self.root, &word, 0)
    }

    fn _search(&self, node: &TrieNode, word: &str, index: usize) -> bool {
        if index == word.len() {
            return node.is_word;
        }

        let c = word.chars().nth(index).unwrap();
        if c == '.' {
            return node
                .children
                .values()
                .any(|child| self._search(child, word, index + 1));
        }
        if let Some(child) = node.children.get(&c) {
            return self._search(child, word, index + 1);
        }

        false
    }
}

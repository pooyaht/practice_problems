struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_word: bool,
    max_index: i32,
}

impl TrieNode {
    fn new() -> TrieNode {
        let children = (0..27).map(|_| -> Option<Box<TrieNode>> { None }).collect();
        Self {
            children,
            is_word: false,
            max_index: 0,
        }
    }

    fn insert(&mut self, word: &str, index: i32) {
        let mut node = self;
        for c in word.chars() {
            let i = (c as usize) - ('a' as usize);
            node = node.children[i].get_or_insert_with(|| Box::new(TrieNode::new()));
            node.max_index = node.max_index.max(index);
        }
        node.max_index = node.max_index.max(index);
        node.is_word = true;
    }

    fn search(&self, word: &str) -> i32 {
        let mut node = self;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            if let Some(child) = &node.children[index] {
                node = child;
            } else {
                return -1;
            }
        }
        node.max_index
    }
}

struct WordFilter {
    trie_root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut root = TrieNode::new();
        for (index, word) in words.iter().enumerate() {
            for i in 0..word.len() {
                root.insert(&format!("{}{}{}", &word[i..], "{", &word), index as i32)
            }
        }
        WordFilter { trie_root: root }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let query = format!("{}{}{}", &suff, "{", &pref);
        self.trie_root.search(&query)
    }
}

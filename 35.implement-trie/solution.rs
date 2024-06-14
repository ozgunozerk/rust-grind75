use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_leaf: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        word.chars()
            .fold(self, |node, c| node.children.entry(c).or_default())
            .is_leaf = true;
    }

    fn search(&self, word: String) -> bool {
        self._get(word).map_or(false, |node| node.is_leaf)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self._get(prefix).is_some()
    }

    fn _get(&self, c: String) -> Option<&Trie> {
        c.chars().try_fold(self, |node, c| node.children.get(&c))
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

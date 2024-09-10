use std::collections::{hash_map::Entry, HashMap};

/// `Node`s do not move in the `store` vector, but their `next` and `prev` pointers change
///
/// this is not exactly a LinkedList implementation, since `Node`s are stored in a vector,
/// and instead of having the `next` pointer pointed to the next `Node`,
/// it is pointing to the index of the next node in the vector. The same goes for the `prev` pointer.
struct Node {
    key: i32,    // the key of the node (this is not the index!)
    val: i32,    // the value of the node (this is not the index!)
    prev: usize, // index of the previous node in the store
    next: usize, // index of the next node in the store
}

// `LRUCache` is a doubly linked list with a hash map to enable O(1) access to items in the `store` vector.
struct LRUCache {
    first: usize,             // index of the oldest item in the store
    last: usize,              // index of the newest item in the store
    store: Vec<Node>,         // store acts like a doubly linked list
    map: HashMap<i32, usize>, // mapping the `key` of `Node`s to their indices in the `store` vector
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            store: Vec::with_capacity(capacity as usize),
            first: 0,
            last: 0,
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let Some(&i) = self.map.get(&key) else {
            // item not found
            return -1;
        };

        // item already at `last` (most recently used), nothing to do
        if i == self.last {
            return self.store[i].val;
        }

        // we don't move Nodes in the store vector,
        // to make the Node `last` item (newest), we need to modify the related nodes `prev` and `next` pointers
        // and also modify the `first` pointer if necessary
        if i != self.first {
            // if the item is not at the `first` position (oldest item)
            // then it does have `prev` pointer, and we need to update the prev's next pointer
            let prev = self.store[i].prev;
            self.store[prev].next = self.store[i].next;
        } else {
            // item is currently at the `first` position (oldest item)
            // this item is going to be the `last` item (newest)
            // so, the new `first` item should be the next item of the current item
            self.first = self.store[i].next;
        }
        // modifying the `next`s `prev` pointer
        let next = self.store[i].next;
        self.store[next].prev = self.store[i].prev;

        // the old `last` item should be point to the new `last` item with its `next` pointer
        self.store[self.last].next = i;
        // new `last` item should point to the old `last` item with its `prev` pointer
        self.store[i].prev = self.last;
        self.last = i;

        self.store[i].val
    }

    fn put(&mut self, key: i32, val: i32) {
        match self.map.entry(key) {
            Entry::Occupied(o) => {
                // update the value of the existing key
                self.store[*o.get()].val = val;
                // "move" the item to the `last` position (newest) by calling `get`
                self.get(key);
            }
            Entry::Vacant(v) => {
                // key does not exist yet

                // if the store vector is not full, just insert the new Node at the end of the store vector
                if self.store.capacity() > self.store.len() {
                    // create a new Node and insert it at the end of the store vector
                    self.store.push(Node {
                        key,
                        val,
                        prev: self.last,
                        next: 0,
                    });
                    // index of the new node is the length - 1
                    let i = self.store.len() - 1;
                    // update the pointers of the old `last` Node
                    self.store[self.last].next = i;
                    self.last = i;
                    v.insert(i);
                    return;
                }

                // store vector is full, and we cannot just `remove` the oldest one from the `store`,
                // because then indices of the other Nodes in the `store` would change.
                // so, we will add our new item by overwriting the oldest item,
                // and then calling `get` on it to "move" it to the `last` position (newest).

                // new index of should be the index of the oldest item
                v.insert(self.first);

                // remove it from the map
                self.map.remove(&self.store[self.first].key);

                // overwrite old `first` Node fields with the new one
                self.store[self.first].key = key;
                self.store[self.first].val = val;
                self.get(key);
            }
        }
    }
}

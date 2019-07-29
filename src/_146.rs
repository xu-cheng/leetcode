use std::collections::HashMap;

#[derive(Debug)]
struct LRUCacheNode {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug)]
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    head: Option<usize>,
    tail: Option<usize>,
    arena: Vec<LRUCacheNode>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        Self {
            capacity: cap,
            map: HashMap::new(),
            head: None,
            tail: None,
            arena: Vec::with_capacity(cap),
        }
    }
    fn move_node_to_head(&mut self, addr: usize) {
        let (prev, next) = {
            let node = &self.arena[addr];
            (node.prev, node.next)
        };
        if let Some(prev) = prev {
            self.arena[prev].next = next;
        } else {
            return;
        }
        if let Some(next) = next {
            self.arena[next].prev = prev;
        } else {
            self.tail = prev;
        }
        self.arena[self.head.unwrap()].prev = Some(addr);
        let node = &mut self.arena[addr];
        node.prev = None;
        node.next = self.head;
        self.head = Some(addr);
    }
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&addr) = self.map.get(&key) {
            self.move_node_to_head(addr);
            self.arena[addr].value
        } else {
            -1
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&addr) = self.map.get(&key) {
            self.move_node_to_head(addr);
            self.arena[addr].value = value;
        } else if self.arena.len() < self.capacity {
            let addr = self.arena.len();
            self.map.insert(key, addr);
            if let Some(head_addr) = self.head {
                self.arena[head_addr].prev = Some(addr);
            }
            self.arena.push(LRUCacheNode {
                key,
                value,
                prev: None,
                next: self.head,
            });
            self.head = Some(addr);
            if self.tail.is_none() {
                self.tail = Some(addr);
            }
        } else {
            let addr = self.tail.unwrap();
            let node = &mut self.arena[addr];
            self.map.insert(key, addr);
            self.map.remove(&node.key);
            node.key = key;
            node.value = value;
            self.move_node_to_head(addr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);

        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);
        cache.put(2, 2);
        assert_eq!(cache.get(3), -1);
        assert_eq!(cache.get(2), 2);
    }
}

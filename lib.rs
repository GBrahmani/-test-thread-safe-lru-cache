
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub struct LruCache<K, V> {
    capacity: usize,
    map: RwLock<HashMap<K, V>>,
    order: RwLock<VecDeque<K>>,
}

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            capacity,
            map: RwLock::new(HashMap::new()),
            order: RwLock::new(VecDeque::new()),
        })
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let result = {
            let map = self.map.read().unwrap();
            map.get(key).cloned()
        };
        if result.is_some() {
            let mut order = self.order.write().unwrap();
            order.retain(|k| k != key);
            order.push_front(key.clone());
        }
        result
    }

    pub fn put(&self, key: K, value: V) {
        {
            let mut map = self.map.write().unwrap();
            map.insert(key.clone(), value);
        }
        let mut order = self.order.write().unwrap();
        order.retain(|k| *k != key);
        order.push_front(key.clone());
        if order.len() > self.capacity {
            if let Some(old_key) = order.pop_back() {
                let mut map = self.map.write().unwrap();
                map.remove(&old_key);
            }
        }
    }

    pub fn len(&self) -> usize { self.order.read().unwrap().len() }
    pub fn capacity(&self) -> usize { self.capacity }
}

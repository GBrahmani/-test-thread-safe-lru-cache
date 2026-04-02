
#[cfg(test)]
mod tests {
    use super::super::LruCache;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic_put_get() {
        let cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        assert_eq!(cache.get(&1), Some("a"));
        assert_eq!(cache.get(&2), Some("b"));
    }

    #[test]
    fn test_eviction() {
        let cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(3, "c");
        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn test_concurrent() {
        let cache = LruCache::new(5);
        let shared = Arc::clone(&cache);
        let t1 = thread::spawn({ let c = Arc::clone(&shared); move || { for i in 0..100 { c.put(i, i); }}});
        let t2 = thread::spawn({ let c = Arc::clone(&shared); move || { for i in 0..100 { let _ = c.get(&i); }}});
        t1.join().unwrap();
        t2.join().unwrap();
        assert!(shared.len() <= 5);
    }
}

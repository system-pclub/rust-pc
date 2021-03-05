use std::sync::{ Arc, RwLock, RwLockReadGuard };
use std::collections::HashMap;
use std::collections::hash_map::Iter;

type SimpleCollection = HashMap<String, String>;

struct Store(Arc<RwLock<SimpleCollection>>);

impl Store {
    fn new() -> Store { return Store(Arc::new(RwLock::new(SimpleCollection::new()))) }

    fn get(&self, key: &str) -> Option<String> {
        let map = self.0.read().unwrap();
        return map.get(&key.to_string()).map(|s| s.clone());
    }

    fn set(&self, key: &str, value: &str) {
        let mut map = self.0.write().unwrap();
        map.insert(key.to_string(), value.to_string());
    }
    
    fn lock(&self) -> StoreLock {
        StoreLock { guard: self.0.read().unwrap() }
    }
}

struct StoreLock<'a> {
    guard: RwLockReadGuard<'a, SimpleCollection>,
}

impl<'a> StoreLock<'a> {
    fn scan(&self) -> Cursor {
        Cursor { iter: self.guard.iter() }
    }
}

struct Cursor<'l> {
    iter: Iter<'l, String, String>,
}

impl<'l> Cursor<'l> {
    fn next(&mut self) -> Option<(String, String)> {
        return self.iter.next().map(|r| (r.0.clone(), r.1.clone()));
    }
}

fn main() {
    let s = Store::new();
    s.set("x", "10");
    s.set("y", "20");
    s.set("z", "30");
    // usage
    let lock = s.lock();
    let _c = lock.scan();
}


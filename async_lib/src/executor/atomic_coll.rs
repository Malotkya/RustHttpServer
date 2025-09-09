use std::{
    ops::Deref,
    sync::{Mutex, Arc},
    collections::{VecDeque, BTreeMap}
};

pub(crate) struct Queue<T>(Arc<Mutex<VecDeque<T>>>, &'static str);

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1
        )
    }
}

impl<T> Queue<T> {
    pub fn new(name:&'static str, capacity:usize) -> Self {
            Self(Arc::new(
                Mutex::new(
                    VecDeque::with_capacity(capacity)
                )
            ),
            name
        )
    }

    pub fn push(&self, item:T) {
        let mut queue = self.0.lock().unwrap();
        if queue.len() >= queue.capacity() {
            panic!("{} queue is full!", self.1)
        }
        queue.push_back(item);
    }

    pub fn pop(&self) -> Option<T> {
        let mut queue = self.0.lock().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.0.lock().unwrap();
        queue.is_empty()
    }
}

pub(crate) struct Map<K: Ord, V:Send>(Arc<Mutex<BTreeMap<K, Arc<V>>>>);

impl<K: Ord, V: Send> Clone for Map<K, V> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone()
        )
    }
}

impl<K: Ord, V:Send> Map<K, V> {
    pub fn new() -> Self {
        Self ( Arc::new(
            Mutex::new(
                BTreeMap::new()
            )
        ))
    }

    pub fn default_entry(&self, key:K, callback: impl FnOnce() -> V) -> Arc<V> {
        let mut map = self.0.lock().unwrap();
        map.entry(key).or_insert_with(||{
            Arc::new(callback())
        }).clone()
    }

    pub fn get(&self, key:&K) -> Option<Arc<V>> {
        let map = self.0.lock().unwrap();
        map.get(key).map(|e|e.clone())
    }

    pub fn insert(&self, key:K, value:V) -> Option<Arc<V>> {
        let mut map = self.0.lock().unwrap();
        map.insert(key, Arc::new(value))
    }

    pub fn remove(&self, key:&K) -> Option<Arc<V>> {
        let mut map = self.0.lock().unwrap();
        map.remove(key)
    }
}

impl<K: Ord, V:Send + Clone> Map<K, V> {
    pub fn get_mut(&self, key:&K) -> Option<Arc<Mutex<V>>> {
        let map = self.0.lock().unwrap();
        map.get(key).map(|e|{
            let m = Mutex::new((*e.deref()).clone());
            Arc::new(m)
        })
    }
}
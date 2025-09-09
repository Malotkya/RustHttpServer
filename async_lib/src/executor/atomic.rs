use std::{
    ops::Deref,
    sync::{Mutex, Arc},
    collections::{VecDeque, BTreeMap},
    task::{Context, Poll},
    pin::Pin
};

pub(crate) struct AtomicQueue<T>(Arc<Mutex<VecDeque<T>>>, &'static str);

impl<T> Clone for AtomicQueue<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1
        )
    }
}

impl<T> AtomicQueue<T> {
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

impl<T: PartialEq> AtomicQueue<T> {
    pub fn unique_push(&self, value: T) {
        let mut queue = self.0.lock().unwrap();

        for item in queue.iter() {
            if value == *item {
                return;
            }
        }

        queue.push_back(value);
    }
}

pub(crate) struct AtomicMap<K: Ord, V:Send>(Arc<Mutex<BTreeMap<K, Arc<V>>>>);

impl<K: Ord, V: Send> Clone for AtomicMap<K, V> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone()
        )
    }
}

impl<K: Ord, V:Send> AtomicMap<K, V> {
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

impl<K: Ord, V:Send + Clone> AtomicMap<K, V> {
    pub fn get_mut(&self, key:&K) -> Option<Arc<Mutex<V>>> {
        let map = self.0.lock().unwrap();
        map.get(key).map(|e|{
            let m = Mutex::new((*e.deref()).clone());
            Arc::new(m)
        })
    }
}

#[derive(Clone)]
pub(crate) struct AtomicFuture<'a>(Arc<Mutex<Pin<Box<dyn Future<Output = ()> + 'a>>>>);

impl<'a> AtomicFuture<'a> {
    pub fn new(f: impl Future<Output = ()> + 'a) -> Self {
        Self(
            Arc::new(
                Mutex::new(
                    Box::pin(f)
                )
            )
        )
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        let mut task = self.0.lock().unwrap();
        task.as_mut().poll(cx)
    }
}

#[derive(Clone)]
pub(crate) struct AtomicOption<T:Send>(Arc<Mutex<Option<Arc<T>>>>);

impl<T:Send> AtomicOption<T> {
    pub fn new() -> Self {
        Self::from(None)
    }

    pub fn from(value: Option<T>) -> Self {
        Self(
            Arc::new(Mutex::new(
                value.map(|v|Arc::new(v))
            ))
        )
    }

    pub fn is_some(&self) -> bool {
        let option = self.0.lock().unwrap();
        option.is_some()
    } 

    pub fn is_none(&self) -> bool {
        let option = self.0.lock().unwrap();
        option.is_none()
    } 

    pub fn unwrap(&self) -> Arc<T> {
        let option = self.0.lock().unwrap();
        option.clone().unwrap()
    }

    pub fn try_unwrap(&self) -> Option<Arc<T>> {
        let option = self.0.lock().unwrap();
        option.clone()
    }

    pub fn set(&self, value:Option<T>) {
        let mut option = self.0.lock().unwrap();
        *option = value.map(|v|Arc::new(v));
    }
}
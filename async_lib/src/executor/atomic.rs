#![allow(dead_code)]
use std::{
    ops::Deref,
    sync::{Mutex, Arc, atomic::AtomicPtr},
    collections::{VecDeque, BTreeMap},
    task::{Context, Poll},
    pin::Pin
};

#[derive(Debug)]
pub(crate) struct AtomicQueue<T>(Arc<Mutex<VecDeque<T>>>, &'static str);

//SAFTEY: Arc<Mutex<T>>
unsafe impl<T> Send for AtomicQueue<T> {}
unsafe impl<T> Sync for AtomicQueue<T> {}

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

    pub fn set_capacity(&self, value:usize) {
        let mut queue = self.0.lock().unwrap();

        let update = (value as i64) - (queue.capacity() as i64);
        if update <= 0 {
            return;
        }
            
        queue.reserve(update as usize);
    }

    pub fn push(&self, item:T) {
        let mut queue = self.0.lock().unwrap();
        if queue.len() >= queue.capacity() {
            panic!("{} Queue is full!", self.1)
        }
        queue.push_back(item);
    }

    pub fn pop(&self) -> Option<T> {
        let mut queue = self.0.lock().unwrap();
        queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
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

#[derive(Debug)]
pub(crate) struct AtomicMap<K: Ord, V>(Arc<Mutex<BTreeMap<K, Arc<V>>>>);

//SAFTEY: Arc<Mutex<T>>
unsafe impl<K:Ord, V> Send for AtomicMap<K, V> {}
unsafe impl<K:Ord, V> Sync for AtomicMap<K, V> {}

impl<K:Ord, V> Clone for AtomicMap<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
pub(crate) struct AtomicFuture<T>(Arc<Mutex<Pin<Box<dyn Future<Output = T> + 'static>>>>);

//SAFTEY: Arc<Mutex<T>>
unsafe impl<T> Send for AtomicFuture<T> {}
unsafe impl<T> Sync for AtomicFuture<T> {}

impl<T> AtomicFuture<T> {
    pub fn new(f: impl Future<Output = T> + 'static) -> Self {
        Self(
            Arc::new(
                Mutex::new(
                    Box::pin(f)
                )
            )
        )
    }
}

impl<T> Future for AtomicFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut future = self.0.lock().unwrap();
        future.as_mut().poll(cx)
    }
}

pub(crate) struct AtomicOption<T>(Arc<Mutex<Option<Arc<T>>>>);

//SAFTEY: Arc<Mutex<T>>
unsafe impl<T> Send for AtomicOption<T> {}
unsafe impl<T> Sync for AtomicOption<T> {}

impl<T> Clone for AtomicOption<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
        )
    }
}

impl<T> AtomicOption<T> {
    pub fn none() -> Self {
        Self::from(None)
    }

    pub fn some(value:T) -> Self {
        Self::from(Some(value))
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

pub(crate) struct AtomicList<T>(Arc<Mutex<Vec<T>>>, &'static str);

//SAFTEY: Arc<Mutex<T>>
unsafe impl<T> Send for AtomicList<T> {}
unsafe impl<T> Sync for AtomicList<T> {}

impl<T> Clone for AtomicList<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1
        )
    }
}

impl<T> AtomicList<T> {
    pub fn new(name:&'static str, capacity:usize) -> Self {
            Self(Arc::new(
                Mutex::new(
                    Vec::with_capacity(capacity)
                )
            ),
            name
        )
    }

    pub fn set_capacity(&self, value:usize) {
        let mut list = self.0.lock().unwrap();

        let update = (value as i64) - (list.capacity() as i64);
        if update <= 0 {
            return;
        }
            
        list.reserve(update as usize);
    }

    pub fn get_capacity(&self) -> usize {
        let list = self.0.lock().unwrap();
        list.capacity()
    }

    pub fn push(&self, item:T) {
        let mut list = self.0.lock().unwrap();
        if list.len() >= list.capacity() {
            panic!("{} list is full!", self.1)
        }
        list.push(item);
    }

    pub fn pop(&self) -> Option<T> {
        let mut list = self.0.lock().unwrap();
        list.pop()
    }

    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        let list = self.0.lock().unwrap();
        list.is_empty()
    }

    pub fn get(&self, index:usize) -> Option<AtomicPtr<T>> {
        let list = self.0.lock().unwrap();
        list.get(index).map(|value|AtomicPtr::new(value as *const T as *mut T))
    }
}
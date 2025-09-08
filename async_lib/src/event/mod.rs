use std::{
    collections::HashMap,
    str::FromStr,
    ops::{Deref, DerefMut}
};

mod listener;
use listener::Listener;

pub struct EventEmitterWrapper<T>(EventMap, T);
pub struct EventEmitter(EventMap);

impl<T> Deref for EventEmitterWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> DerefMut for EventEmitterWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl EventEmitter {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn on<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, callback:E) -> String {
        add_listener(&mut self.0, event, None, callback)
    }

    pub fn once<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, callback:E) -> String {
       add_listener(&mut self.0, event, Some(1), callback)
    }

    pub fn on_limited<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, limit:Option<usize>, callback:E) -> String {
        add_listener(&mut self.0, event, limit, callback)
    }

    pub fn remove_listener(&mut self, id: &str) -> bool {
        remove_listener(&mut self.0, id)
    }

    pub fn emit<T: ToString>(&mut self, event: &str, value: T) {
        parse_event(&mut self.0, event, value);
    }
}

impl<T> EventEmitterWrapper<T> {
    pub fn new(inner:T) -> Self {
        Self(
            HashMap::new(),
            inner
        )
    }

    pub fn on<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, callback:E) -> String {
        add_listener(&mut self.0, event, None, callback)
    }

    pub fn once<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, callback:E) -> String {
       add_listener(&mut self.0, event, Some(1), callback)
    }

    pub fn on_limited<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, limit:Option<usize>, callback:E) -> String {
        add_listener(&mut self.0, event, limit, callback)
    }

    pub fn remove_listener(&mut self, id: &str) -> bool {
        remove_listener(&mut self.0, id)
    }

    pub fn emit<A: ToString>(&mut self, event: &str, value: A) {
        parse_event(&mut self.0, event, value);
    }
}

type EventMap = HashMap<String, Vec<Listener>>;

fn add_listener<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(map: &mut EventMap, event: &str, limit:Option<usize>, callback:E) -> String {
    let listener = Listener::new(callback, limit);
    let id = listener.id.clone();

    match map.get_mut(event) {
        Some(list) => list.push(listener),
        None => {map.insert(event.to_string(), vec![listener]);}
    };

    id
}

fn remove_listener(map: &mut EventMap, id: &str) -> bool {
    for(_, list) in map.iter_mut() {
        if let Some(index) = list.iter().position(|listener| listener.id == id) {
            list.remove(index);
            return true;
        }
    }

    return false;
}

fn parse_event<T: ToString>(map: &mut EventMap, event: &str, value: T) {
    if let Some(list) = map.get_mut(event) {
        let mut to_remove:Vec<usize> = Vec::new();

        for (index, listener) in list.iter_mut().enumerate() {
            if listener.call(&value) {
                to_remove.push(index);
            }
        }

        for index in to_remove.iter().rev() {
            list.remove(*index);
        }
    }
}
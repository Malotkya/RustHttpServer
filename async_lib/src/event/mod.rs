use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex}
};
pub use listener::*;

mod listener;

pub trait EventEmitter {
    fn on(&mut self, event:&str, callback: impl EventListener) -> String;
    fn on_async(&mut self, event:&str, callback: impl AsyncEventListener) -> String;

    fn once(&mut self, event:&str, callback: impl EventListener) -> String;
    fn once_async(&mut self, event:&str, callback: impl AsyncEventListener) -> String;

    fn on_limited(&mut self, event:&str, callback: impl EventListener, limit:usize) -> String;
    fn on_limited_async(&mut self, event:&str, callback: impl AsyncEventListener, limit:usize) -> String;

    fn remove_listener(&mut self, id: &str) -> bool;
    fn emit<Event: ToString, Args:ToString>(&mut self, event:Event, args:Args);
}

pub struct EventEmitterWrapper<T>(EventMap, T);

//Event Emiter Wrapper is safe for moving between threads if T is aswell
unsafe impl<T:Sync> Sync for EventEmitterWrapper<T> {}
unsafe impl<T:Send> Send for EventEmitterWrapper<T> {}

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

impl<T> EventEmitterWrapper<T> {
    pub fn new(inner:T) -> Self {
        Self(
            new_emmiter(),
            inner
        )
    }
}

impl<T> EventEmitter for EventEmitterWrapper<T> {
    fn on(&mut self, event: &str, callback: impl EventListener) -> String {
        add_listener(&mut self.0, event, None, callback)
    }

    fn on_async(&mut self, event: &str, callback: impl AsyncEventListener) -> String {
        add_async_listener(&mut self.0, event, None, callback)
    }

    fn once(&mut self, event: &str, callback: impl EventListener) -> String {
       add_listener(&mut self.0, event, Some(1), callback)
    }

    fn once_async(&mut self, event: &str, callback: impl AsyncEventListener) -> String {
        add_async_listener(&mut self.0, event, Some(1), callback)
    }

    fn on_limited(&mut self, event: &str, callback: impl EventListener, limit:usize) -> String {
        add_listener(&mut self.0, event, Some(limit), callback)
    }

    fn on_limited_async(&mut self, event: &str, callback: impl AsyncEventListener, limit:usize) -> String {
        add_async_listener(&mut self.0, event, Some(limit), callback)
    }

    fn remove_listener(&mut self, id: &str) -> bool {
        remove_listener(&mut self.0, id)
    }

    fn emit<Event: ToString, Args:ToString>(&mut self, event: Event, args: Args) {
        let args = args.to_string();
        let event = event.to_string();

        crate::spawn_task(
            parse_event(self.0.clone(), event, args)
        );
    }
}

impl EventEmitter for EventMap {
    fn on(&mut self, event: &str, callback: impl EventListener) -> String {
        add_listener(self, event, None, callback)
    }

    fn on_async(&mut self, event: &str, callback: impl AsyncEventListener) -> String {
        add_async_listener(self, event, None, callback)
    }

    fn once(&mut self, event: &str, callback: impl EventListener) -> String {
       add_listener(self, event, Some(1), callback)
    }

    fn once_async(&mut self, event: &str, callback: impl AsyncEventListener) -> String {
        add_async_listener(self, event, Some(1), callback)
    }

    fn on_limited(&mut self, event: &str, callback: impl EventListener, limit:usize) -> String {
        add_listener(self, event, Some(limit), callback)
    }

    fn on_limited_async(&mut self, event: &str, callback: impl AsyncEventListener, limit:usize) -> String {
        add_async_listener(self, event, Some(limit), callback)
    }

    fn remove_listener(&mut self, id: &str) -> bool {
        remove_listener(self, id)
    }

    fn emit<Event: ToString, Args:ToString>(&mut self, event: Event, args: Args) {
        let args = args.to_string();
        let event = event.to_string();

        crate::spawn_task(
            parse_event(self.clone(), event, args)
        );
    }
}

pub(crate) fn new_emmiter() -> EventMap {
    Arc::new(Mutex::new(HashMap::new()))
}

pub(crate) type EventMap = Arc<Mutex<HashMap<String, Vec<ListenerItem>>>>;

fn add_listener(cell: &EventMap, event: &str, limit:Option<usize>, callback:impl EventListener) -> String {
    let listener = ListenerItem::new_sync(callback, limit);
    let id = listener.id.clone();
    let mut map = cell.lock().unwrap();

    match map.get_mut(event) {
        Some(list) => list.push(listener),
        None => {map.insert(event.to_string(), vec![listener]);}
    };

    id
}

fn add_async_listener(cell: &EventMap, event: &str, limit:Option<usize>, callback:impl AsyncEventListener) -> String {
    let listener = ListenerItem::new_async(callback, limit);
    let id = listener.id.clone();
    let mut map = cell.lock().unwrap();

    match map.get_mut(event) {
        Some(list) => list.push(listener),
        None => {map.insert(event.to_string(), vec![listener]);}
    };

    id
}

fn remove_listener(cell: &EventMap, id: &str) -> bool {
    let mut map = cell.lock().unwrap();

    for(_, list) in map.iter_mut() {
        if let Some(index) = list.iter().position(|listener| listener.id == id) {
            list.remove(index);
            return true;
        }
    }

    return false;
}

async fn parse_event(cell:EventMap, event:String, value: String) {
    let mut map = cell.lock().unwrap();

    if let Some(list) = map.get_mut(&event) {
        let mut to_remove:Vec<usize> = Vec::new();

        for (index, listener) in list.iter_mut().enumerate() {
            
            if listener.call(&value).await {
                to_remove.push(index);
            }
        }

        for index in to_remove.iter().rev() {
            list.remove(*index);
        }
    }
}
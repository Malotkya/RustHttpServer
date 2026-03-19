use std::pin::Pin;

pub trait EventListener = Fn(&str) + Sync + Send + 'static;
pub trait AsyncEventListener = (Fn(&str)-> Pin<Box<dyn Future<Output = ()>>>) + Sync + Send + 'static;

pub(crate) enum ListenerType {
    Sync(Box<dyn EventListener>),
    Async(Box<dyn AsyncEventListener>)
}

impl ListenerType {
    fn _sync(listener: impl EventListener) -> Self {
        Self::Sync(Box::new(listener))
    }

    fn _async(listner: impl AsyncEventListener) -> Self {
        Self::Async(Box::new(listner))
    }

    async fn call(&self, args:&str) {
        match self {
            Self::Async(fun) => fun(args).await,
            Self::Sync(fun) => fun(args),
        }
    }
}

pub(crate) struct ListenerItem {
    func: ListenerType,
    limit: Option<usize>,
    pub(crate) id: String
}

impl ListenerItem {
    pub fn new_sync(listener: impl EventListener, limit:Option<usize>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            limit,
            func: ListenerType::_sync(listener)
        }
    }

    pub fn new_async(listener: impl AsyncEventListener, limit:Option<usize>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            limit,
            func: ListenerType::_async(listener)
        }
    }

    pub async fn call(&mut self, value:&str) -> bool {
        if let Some(limit) = self.limit {
            if limit == 0 {
                return true;

            } else {
                self.limit = Some(limit - 1);
            }
        }

        self.func.call(value).await;

        return false;
    }
}
























use std::{
    rc::Rc,
    str::FromStr,
    task::{Context, Poll},
    pin::Pin
};

pub struct Listener {
    callback: Rc<dyn Fn(String) + Sync + Send + 'static>,
    limit: Option<usize>,
    pub(crate) id: String
}

struct ListenerTask {
    callback: Rc<dyn Fn(String) + Sync + Send + 'static>,
    arg: String
}

impl Future for ListenerTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        (self.callback)(self.arg.clone());
        Poll::Ready(())
    }
}

impl Listener {
    pub fn new<T: FromStr + 'static, E:Fn(T) + Sync + Send + 'static>(e: E, limit:Option<usize>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            limit,
            callback: Rc::new(move |str:String| {
                match str.parse() {
                    Ok(value) => e(value),
                    Err(_) => print!("An error occured when parsing {}!", str)
                }
            })
        }
    }

    pub fn call<T: ToString>(&mut self, value:&T) -> bool {
        if let Some(limit) = self.limit {
            if limit == 0 {
                return true;

            } else {
                self.limit = Some(limit - 1);
            }
        }
        //let string = value.to_string();
        let arg = value.to_string();
        let callback = Rc::clone(&self.callback);
        crate::spawn_task(ListenerTask{
            arg, callback
        });

        return false;
    }
}
























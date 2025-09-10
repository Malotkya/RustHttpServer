use std::{
    net::TcpStream, 
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, TryRecvError},
        Arc, Mutex
    },
    thread::{sleep, JoinHandle},
    time::Duration
};

pub(crate) type ConnectionError = TryRecvError;
pub struct ListenerHandle(
    pub(crate) Arc<AtomicBool>, 
    pub(crate) JoinHandle<()>,
    pub(crate) Receiver<TcpStream>
);

impl Deref for ListenerHandle {
    type Target = JoinHandle<()>;

    fn deref(&self) -> &Self::Target {
        &self.1 
    }
}

impl ListenerHandle {
    pub fn close(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn wait_close(self)->Result<(), Box<dyn std::any::Any + Send + 'static>> {
        self.close();
        self.1.join()
    }

    pub fn conn(&self) -> Result<TcpStream, ConnectionError> {
        self.2.try_recv()
    }
}

pub(crate) struct Ready(
    Arc<Mutex<
        Result<
            Option<std::io::Error>,
            ()
        >
    >>
);

impl Ready {
    pub fn new() -> (Self, Self) {
        let mutex = Arc::new(Mutex::new(Err(())));

        (
            Self(mutex.clone()),
            Self(mutex)
        )
    }

    pub fn set_success(&self) {
        let mut value = self.0.lock().unwrap();
        *value = Ok(None);
    }

    pub fn set_error(&self, err: std::io::Error) {
        let mut result = self.0.lock().unwrap();
        *result = Ok(Some(err));
    }

    pub fn wait(self) -> Result<(), std::io::Error>{
        loop {
            let result = self.0.as_ref().lock().unwrap();
            if let Ok(err) = result.as_ref() {
                if let Some(err) = err {
                    return Err(clone_err(err));
                } else {
                    return Ok(());
                }
            }

            sleep(Duration::from_nanos(5));
        }
    } 
}

#[inline]
fn clone_err(e: &std::io::Error) -> std::io::Error {
    std::io::Error::new(
        e.kind().clone(),
        e.to_string()
    )
}
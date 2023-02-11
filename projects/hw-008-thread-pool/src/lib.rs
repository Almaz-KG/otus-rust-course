use std::fmt::{Display, Formatter};
use std::mem;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

#[derive(Debug)]
pub enum ThreadPoolError {
    SpawnError(String),
    InternalError,
    Unknown,
}

impl Display for ThreadPoolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreadPoolError::SpawnError(msg) => { f.write_str(&format!("[SpawnError]: {msg}")) }
            ThreadPoolError::InternalError => { f.write_str(&format!("[InternalError]")) }
            ThreadPoolError::Unknown => { f.write_str(&format!("[Unknown]")) }
        }
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct SizedThreadPool {
    pub pool_size: usize,
    join_handlers: Arc<Vec<JoinHandle<()>>>,
    sender: Option<Sender<Task>>,
}

impl SizedThreadPool {
    pub fn new(pool_size: usize) -> Result<Self, ThreadPoolError> {
        fn get_task<Task>(receiver: &Arc<Mutex<Receiver<Task>>>) -> Result<Task, RecvError> {
            receiver.lock().unwrap().recv()
        }

        if pool_size != 0 {
            let (sender, receiver) = mpsc::channel::<Task>();
            let receiver = Arc::new(Mutex::new(receiver));

            let join_handlers = (0..pool_size)
                .map(|index| {
                    let rc = receiver.clone();
                    std::thread::spawn(move || {
                        let thread_name = format!("[Thread-{index:0>2}]");
                        while let Ok(task) = get_task(&rc) {
                            println!("{thread_name}: started executing the task");
                            let now = Instant::now();
                            {
                                task();
                            }
                            println!("{thread_name}: processing time {:.2?}", now.elapsed());
                        }
                    })
                })
                .collect();

            Ok(Self {
                pool_size,
                join_handlers: Arc::new(join_handlers),
                sender: Some(sender),
            })
        } else {
            Err(ThreadPoolError::SpawnError(
                "Thread pools size should be greater than 0".to_string(),
            ))
        }
    }

    pub fn add_task<T>(&self, task: T) -> Result<(), ThreadPoolError>
    where
        T: FnOnce() -> () + Send + 'static,
    {
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(task))
            .map_err(|e| ThreadPoolError::SpawnError(e.to_string()))
    }
}

impl Drop for SizedThreadPool {
    fn drop(&mut self) {
        let arc = mem::replace(&mut self.join_handlers, Default::default());

        if let Ok(handles) = Arc::try_unwrap(arc) {
            mem::drop(self.sender.take());

            for jh in handles {
                jh.join().unwrap();
            }
        }
    }
}

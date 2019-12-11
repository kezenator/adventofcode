use std::fmt::Debug;
use std::collections::VecDeque;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;

struct Inner<T>
    where T: Debug
{
    name: String,
    debug: bool,
    queue: VecDeque<T>,
    tasks: Vec<Waker>,
}

impl<T> Inner<T>
    where T: Debug
{
    fn new(name: String, debug: bool) -> Self
    {
        Inner
        {
            name,
            debug,
            queue: VecDeque::new(),
            tasks: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Sender<T>
    where T: Debug
{
    inner: Arc<Mutex<Inner<T>>>,
}

impl<T> Sender<T>
    where T: Debug
{
    pub fn send(&self, val: T)
    {
        let mut inner = self.inner.lock().unwrap();

        if inner.debug
        {
            println!("{}: TX: {:?}", inner.name, val);
        }

        inner.queue.push_back(val);
        for task in inner.tasks.iter()
        {
            //println!("{}: W+", inner.name);
            task.clone().wake();
        }
        inner.tasks.truncate(0);
    }
}

#[derive(Clone)]
pub struct Receiver<T>
    where T: Debug
{
    inner: Arc<Mutex<Inner<T>>>,
}

impl<T> Receiver<T>
    where T: Debug
{
    pub fn remainder(&self) -> Vec<T>
    {
        let mut inner = self.inner.lock().unwrap();

        if inner.debug
        {
            println!("{}: Remainder: {:?}", inner.name, inner.queue);
        }

        inner.queue.drain(..).collect()
    }
}

impl<T> Future for Receiver<T>
    where T: Debug
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context)-> Poll<Self::Output>
    {
        let mut inner = self.inner.lock().unwrap();
        if inner.queue.is_empty()
        {
            //println!("{}: W-", inner.name);
            inner.tasks.push(cx.waker().clone());
            return Poll::Pending;
        }
        let val = inner.queue.pop_front().unwrap();

        if inner.debug
        {
            println!("{}: RX: {:?}", inner.name, val);
        }
        
        return Poll::Ready(val);
    }
}

pub fn channel<T>(name: String, debug: bool) -> (Sender<T>, Receiver<T>)
    where T: Debug
{
    let inner = Arc::new(Mutex::new(Inner::new(name, debug)));
    let sender = Sender{ inner: inner.clone() };
    let receiver = Receiver{ inner: inner.clone() };
    return (sender, receiver);
}

use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub mod simple_executor;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    // The 'static lifetime is required here because the returned Task can live for an arbitrary time, so the future needs to be valid for that time too
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }
    /// The reason that this method takes self: Pin<&mut Self> instead of the normal &mut self is that
    /// future instances created from async/await are often self-referential, as we saw above. By
    /// wrapping Self into Pin and letting the compiler opt-out of Unpin for self-referential futures 
    /// generated from async/await, it is guaranteed that the futures are not moved in memory between 
    /// poll calls. This ensures that all internal references are still valid
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}


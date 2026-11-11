//! A minimal `block_on`: no runtime, no dependency.

use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs a future to completion on the current thread.
///
/// When the future is pending, the thread sleeps until it is woken. Hosts
/// that answer immediately (tests, the stdio engine) never sleep.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
        thread::park();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_ready_future() {
        assert_eq!(block_on(async { 1 + 1 }), 2);
    }

    #[test]
    fn wakes_from_another_thread() {
        use std::sync::atomic::{AtomicBool, Ordering};
        struct Later(Arc<AtomicBool>, bool);
        impl Future for Later {
            type Output = u8;
            fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
                if self.0.load(Ordering::SeqCst) {
                    return Poll::Ready(7);
                }
                if !self.1 {
                    self.1 = true;
                    let flag = self.0.clone();
                    let w = cx.waker().clone();
                    thread::spawn(move || {
                        flag.store(true, Ordering::SeqCst);
                        w.wake();
                    });
                }
                Poll::Pending
            }
        }
        assert_eq!(block_on(Later(Arc::new(AtomicBool::new(false)), false)), 7);
    }
}

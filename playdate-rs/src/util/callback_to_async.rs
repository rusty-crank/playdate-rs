use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct CallbackFutureSharedState<T> {
    result: Option<T>,
    waker: Option<core::task::Waker>,
}

pub struct CallbackFuture<T> {
    state: Box<spin::Mutex<CallbackFutureSharedState<T>>>,
}

impl<T> CallbackFuture<T> {
    pub fn new() -> Self {
        Self {
            state: Box::new(spin::Mutex::new(CallbackFutureSharedState {
                result: None,
                waker: None,
            })),
        }
    }

    pub fn get_handle(&self) -> *mut core::ffi::c_void {
        self.state.as_ref() as *const spin::Mutex<CallbackFutureSharedState<T>>
            as *mut core::ffi::c_void
    }

    pub fn set_result(&self, result: T) {
        let mut state = self.state.lock();
        state.result = Some(result);
    }

    pub fn resolve(handle: *mut core::ffi::c_void, result: T) {
        let data = handle as *mut spin::Mutex<CallbackFutureSharedState<T>>;
        let state = unsafe { &*data };
        let mut state = state.lock();
        if state.result.is_some() {
            return;
        }
        state.result = Some(result);
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut lock = self.state.lock();
        if let Some(result) = lock.result.take() {
            Poll::Ready(result)
        } else {
            lock.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

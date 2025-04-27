use alloc::boxed::Box;
use alloc::sync::Arc;
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
    state: Arc<spin::Mutex<CallbackFutureSharedState<T>>>,
}

impl<T> CallbackFuture<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(spin::Mutex::new(CallbackFutureSharedState {
                result: None,
                waker: None,
            })),
        }
    }

    pub fn get_handle(&self) -> *mut core::ffi::c_void {
        Box::into_raw(Box::new(self.state.clone())) as *mut core::ffi::c_void
    }

    pub fn set_result(&self, result: T) {
        let mut state = self.state.lock();
        state.result = Some(result);
    }

    pub fn resolve(handle: *mut core::ffi::c_void, result: T) {
        let data = handle as *mut Arc<spin::Mutex<CallbackFutureSharedState<T>>>;
        let state = unsafe { Box::from_raw(data) };
        let mut state = state.lock();
        if state.result.is_some() {
            return;
        }
        state.result = Some(result);
        if let Some(waker) = state.waker.take() {
            core::mem::drop(state);
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

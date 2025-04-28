use alloc::sync::Arc;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};
use futures_task::{waker_ref, ArcWake};

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::task::{Context, Poll, Waker};
use spin::Mutex;

use crate::PLAYDATE;

pub static EXECUTOR: Executor = Executor::new();

pub struct Executor {
    ready_queue: Mutex<VecDeque<Arc<Task>>>,
    frame_epoch: AtomicUsize,
    frame_wakers: Mutex<Vec<Waker>>,
}

unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}

impl Executor {
    const fn new() -> Self {
        Self {
            ready_queue: Mutex::new(VecDeque::new()),
            frame_epoch: AtomicUsize::new(0),
            frame_wakers: Mutex::new(Vec::new()),
        }
    }

    fn poll_next_task(&self) -> Option<Arc<Task>> {
        let mut ready_queue = self.ready_queue.lock();
        if ready_queue.is_empty() {
            None
        } else {
            Some(ready_queue.pop_front().unwrap())
        }
    }

    pub fn run(&self) {
        while let Some(task) = self.poll_next_task() {
            let mut future_slot = task.future.lock();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }

    pub fn spawn(&self, future: impl 'static + Future<Output = ()>) {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(future))),
        });
        self.spawn_task(task);
    }

    fn spawn_task(&self, task: Arc<Task>) {
        self.ready_queue.lock().push_back(task);
    }

    pub fn signal_next_frame(&self) {
        self.frame_epoch.fetch_add(1, Ordering::SeqCst);
        let wakers = core::mem::take(&mut *self.frame_wakers.lock());
        for w in wakers {
            w.wake()
        }
        self.run();
    }

    pub fn next_frame(&self) -> impl Future<Output = f32> {
        let prev_epoch = EXECUTOR.frame_epoch.load(Ordering::SeqCst);
        let prev_time = PLAYDATE.system.get_current_time_milliseconds();
        NextFrameFuture {
            prev_epoch,
            prev_time,
        }
    }

    pub fn sleep(&self, ms: usize) -> impl Future<Output = ()> {
        TimerFuture {
            target: PLAYDATE.system.get_current_time_milliseconds() + ms,
        }
    }

    pub fn yield_now(&self) -> impl Future<Output = ()> {
        YieldFuture { yielded: false }
    }
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()>>>>>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        EXECUTOR.spawn_task(cloned);
    }
}

pub struct NextFrameFuture {
    prev_epoch: usize,
    prev_time: usize,
}

impl Future for NextFrameFuture {
    type Output = f32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let curr_epoch = EXECUTOR.frame_epoch.load(Ordering::SeqCst);
        if curr_epoch > self.prev_epoch {
            let curr_time = PLAYDATE.system.get_current_time_milliseconds();
            let delta = (curr_time - self.prev_time) as f32 / 1000.0;
            Poll::Ready(delta)
        } else {
            EXECUTOR.frame_wakers.lock().push(_cx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct TimerFuture {
    target: usize,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let curr_time = PLAYDATE.system.get_current_time_milliseconds();
        if curr_time >= self.target {
            Poll::Ready(())
        } else {
            EXECUTOR.frame_wakers.lock().push(_cx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct YieldFuture {
    yielded: bool,
}

impl Future for YieldFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            EXECUTOR.frame_wakers.lock().push(_cx.waker().clone());
            Poll::Pending
        }
    }
}

use sys::PDSystemEvent as SystemEvent;

type EventHandler = Box<dyn FnMut(u32)>;

pub struct EventManager {
    handlers: [spin::Mutex<Vec<EventHandler>>; Self::NUM_EVENT_TYPES],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallbackHandle {
    ptr: *const dyn FnMut(u32),
}

impl EventManager {
    pub(crate) const NUM_EVENT_TYPES: usize = 12;

    pub(crate) fn new() -> Self {
        Self {
            handlers: [const { spin::Mutex::new(Vec::new()) }; Self::NUM_EVENT_TYPES],
        }
    }

    pub fn on(&self, event: SystemEvent, handler: impl 'static + FnMut(u32)) -> CallbackHandle {
        let handler: EventHandler = Box::new(handler);
        let handle = CallbackHandle {
            ptr: handler.as_ref() as *const dyn FnMut(u32) -> (),
        };
        self.handlers[event as usize].lock().push(handler);
        handle
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    pub fn off(&self, event: SystemEvent, handle: CallbackHandle) {
        let retain = |h: &EventHandler| h.as_ref() as *const dyn FnMut(u32) -> () != handle.ptr;
        let mut handlers = self.handlers[event as usize].lock();
        handlers.retain(retain);
    }

    pub(crate) fn signal(&self, event: SystemEvent, arg: u32) {
        let mut handlers = self.handlers[event as usize].lock();
        for handler in handlers.iter_mut() {
            let handler = handler.as_mut();
            handler(arg);
        }
    }
}

use crate::PLAYDATE;

pub use crate::sys::accessReply as AccessReply;
pub use crate::sys::PDNetErr;
pub use crate::sys::WifiStatus;

pub mod http;
pub mod tcp;

fn http_handle() -> &'static sys::playdate_http {
    unsafe { &*(*(*PLAYDATE.raw_api).network).http }
}

fn tcp_handle() -> &'static sys::playdate_tcp {
    unsafe { &*(*(*PLAYDATE.raw_api).network).tcp }
}

pub fn get_status() -> WifiStatus {
    let status = unsafe { (*(*PLAYDATE.raw_api).network).getStatus.unwrap()() };
    status
}

/// Playdate will connect to the configured access point automatically as needed and turn off the wifi radio after a 30 second idle timeout. This function allows a game to start connecting to the access point sooner, since that can take upwards of 10 seconds, or turn off wifi as soon as it’s no longer needed instead of waiting 30 seconds. If flag is true, a callback function can be provided to check for an error connecting to the access point.
#[allow(static_mut_refs)]
pub fn set_enabled(handle_err: Option<Box<dyn FnOnce() + Send>>) {
    let has_callback = handle_err.is_some();
    static mut ENABLED_CALLBACK: Option<Box<dyn FnOnce() + Send>> = None;
    if let Some(cb) = handle_err {
        unsafe {
            ENABLED_CALLBACK = Some(cb);
        }
    }
    extern "C" fn callback_impl(_error: sys::PDNetErr) {
        if let Some(cb) = unsafe { ENABLED_CALLBACK.take() } {
            cb();
        }
    }
    unsafe {
        (*(*PLAYDATE.raw_api).network).setEnabled.unwrap()(
            has_callback as _,
            if has_callback {
                Some(callback_impl)
            } else {
                None
            },
        );
    }
}

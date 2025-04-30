use crate::PLAYDATE;

pub use crate::error::NetworkError;
pub use crate::sys::accessReply as AccessReply;
pub use crate::sys::PDNetErr;
pub use crate::sys::WifiStatus;

pub mod http;
pub mod tcp;
pub mod ws;

use alloc::boxed::Box;

fn http_handle() -> &'static sys::playdate_http {
    unsafe { &*(*(*PLAYDATE.raw_api).network).http }
}

fn tcp_handle() -> &'static sys::playdate_tcp {
    unsafe { &*(*(*PLAYDATE.raw_api).network).tcp }
}

pub fn get_status() -> WifiStatus {
    unsafe { (*(*PLAYDATE.raw_api).network).getStatus.unwrap()() }
}

/// Playdate will connect to the configured access point automatically as needed and turn off the wifi radio after a 30 second idle timeout. This function allows a game to start connecting to the access point sooner, since that can take upwards of 10 seconds, or turn off wifi as soon as it’s no longer needed instead of waiting 30 seconds. If flag is true, a callback function can be provided to check for an error connecting to the access point.
#[allow(static_mut_refs)]
pub fn set_enabled(handle_err: Option<impl FnOnce(Option<NetworkError>) + 'static>) {
    let has_callback = handle_err.is_some();
    static mut CALLBACK: Option<Box<dyn FnOnce(Option<NetworkError>)>> = None;
    unsafe {
        CALLBACK = handle_err.map::<Box<dyn FnOnce(Option<NetworkError>)>, _>(|cb| Box::new(cb))
    };
    extern "C" fn callback_impl(e: sys::PDNetErr) {
        if let Some(cb) = unsafe { CALLBACK.take() } {
            let e = if e == NetworkError::OK { None } else { Some(e) };
            cb(e);
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

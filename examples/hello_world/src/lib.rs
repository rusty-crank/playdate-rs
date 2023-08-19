#![no_std]

use playdate_rs_sys::{
    LCDSolidColor, PDStringEncoding, PDSystemEvent, PlaydateAPI, LCD_COLUMNS, LCD_ROWS,
};

#[no_mangle]
unsafe extern "C" fn eventHandler(pd: &mut PlaydateAPI, event: PDSystemEvent, _arg: u32) {
    if event == PDSystemEvent::kEventInit {
        ((*pd.system).setUpdateCallback.unwrap())(
            Some(update),
            pd as *mut PlaydateAPI as *mut ::core::ffi::c_void,
        );
    }
}

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

static mut X: i32 = (400 - TEXT_WIDTH) / 2;
static mut Y: i32 = (240 - TEXT_HEIGHT) / 2;
static mut DX: i32 = 1;
static mut DY: i32 = 2;

unsafe extern "C" fn update(userdata: *mut ::core::ffi::c_void) -> i32 {
    let pd: &mut PlaydateAPI = &mut *(userdata as *mut PlaydateAPI);
    ((*pd.graphics).clear.unwrap())(LCDSolidColor::kColorWhite as _);
    let s = "Hello, world!";
    ((*pd.graphics).drawText.unwrap())(
        s.as_ptr() as _,
        s.len(),
        PDStringEncoding::kASCIIEncoding,
        X,
        Y,
    );
    X += DX;
    Y += DY;
    if X < 0 || X > LCD_COLUMNS as i32 - TEXT_WIDTH {
        DX = -DX;
    }
    if Y < 0 || Y > LCD_ROWS as i32 - TEXT_HEIGHT {
        DY = -DY;
    }
    ((*pd.system).drawFPS.unwrap())(0, 0);
    return 1;
}

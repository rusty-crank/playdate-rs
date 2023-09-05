use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;

use crate::PLAYDATE;

struct PlaydateHeapAllocator;

unsafe impl GlobalAlloc for PlaydateHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = PLAYDATE.system.realloc(0 as _, layout.size());
        ptr as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        PLAYDATE.system.realloc(ptr as *mut _, 0);
    }
}

#[cfg(target_os = "none")]
#[global_allocator]
static GLOBAL: PlaydateHeapAllocator = PlaydateHeapAllocator;

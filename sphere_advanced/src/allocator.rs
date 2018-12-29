#![allow(non_camel_case_types)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

type size_t = usize;
type c_int = i32;

extern {
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn posix_memalign(
        memptr: *mut *mut c_void,
        align: size_t,
        size: size_t,
    ) -> c_int;
}

pub struct MyAllocator;

const MIN_ALIGN: usize = 8;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            malloc(layout.size()) as *mut u8
        } else {
            aligned_malloc(&layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }
}

unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    let mut out = ptr::null_mut();
    let ret = posix_memalign(&mut out, layout.align(), layout.size());
    if ret != 0 {
        ptr::null_mut()
    } else {
        out as *mut u8
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}
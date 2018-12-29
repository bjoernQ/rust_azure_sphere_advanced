#![allow(non_camel_case_types)]

use alloc::boxed::Box;
use alloc::boxed::FnBox;
use core::mem;

type c_ulong = u32;
type pthread_t = c_ulong;
type c_int = i32;
type c_void = core::ffi::c_void;

#[repr(C)]
pub struct pthread_attr_t {
    __size: [u32; 9]
}

extern {
    pub fn pthread_create(native: *mut pthread_t,
                          attr: *const pthread_attr_t,
                          f: extern fn(*mut c_void) -> *mut c_void,
                          value: *mut c_void) -> c_int;

    pub fn pthread_join(native: pthread_t,
                        value: *mut *mut c_void) -> c_int;
}

pub struct Thread {
    pub id: pthread_t
}

impl Thread {

    pub fn spawn<F>(f: F) -> Thread where
        F: FnOnce() -> (), F: 'static
    {
        let mut thread_id:pthread_t = 0;

        // Trait object with a stable address
        let func = Box::new(f) as Box<FnOnce()>;
        // Thin pointer
        let func = Box::new(func);
        // Raw pointer
        let func = Box::into_raw(func);

        unsafe {
            pthread_create(&mut thread_id, core::ptr::null_mut(), thread_start,  func as *mut _);
            mem::forget(func);
        }

        return Thread {
            id: thread_id
        };

        extern fn thread_start(main: *mut c_void) -> *mut c_void {
            unsafe {
                let func = Box::from_raw(main as *mut Box<FnBox()>);
                func();
            }
            core::ptr::null_mut()
        }
    }

    pub fn join(&self) {
        unsafe {
            let thread_id: pthread_t = self.id;
            let ret = pthread_join(thread_id, core::ptr::null_mut());
            mem::forget(self);
            if ret != 0 { panic!(); }
        }
    }

}

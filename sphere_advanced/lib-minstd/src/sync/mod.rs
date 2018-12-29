#![allow(non_camel_case_types)]

use core::cell::UnsafeCell;
use core::mem;

type c_int = i32;
type c_long = i64;

const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;

#[repr(C)]
struct pthread_mutex_t {
    __align: [c_long; 0],
    size: [u8; __SIZEOF_PTHREAD_MUTEX_T],
}

#[repr(C)]
struct pthread_mutexattr_t {
    __align: [c_int; 0],
    size: [u8; __SIZEOF_PTHREAD_MUTEXATTR_T],
}

extern {
    fn pthread_mutex_init(lock: *mut pthread_mutex_t,
                              attr: *const pthread_mutexattr_t) -> c_int;
    fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> c_int;
}

pub struct Mutex {
    lock: UnsafeCell<pthread_mutex_t>
}

impl Mutex {

    pub fn new() -> Mutex {
        let mutex = Mutex {
            lock: unsafe { mem::zeroed() }
        };

        unsafe {
            pthread_mutex_init(mutex.lock.get(), core::ptr::null_mut());
        }

        mutex
    }

    pub fn lock(&self) {
        unsafe { pthread_mutex_lock(self.lock.get()); }
    }

    pub fn unlock(&self) {
        unsafe { pthread_mutex_unlock(self.lock.get()); }
    }
}
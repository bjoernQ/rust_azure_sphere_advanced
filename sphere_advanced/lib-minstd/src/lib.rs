#![feature(lang_items, start, libc)]
#![feature(box_syntax)]
#![feature(dropck_eyepatch)]
#![feature(optin_builtin_traits)]
#![no_std]
#![feature(alloc)]
#![feature(try_from)]
#![feature(uniform_paths)]
#![feature(int_error_internals)]
#![feature(slice_internals)]
#![feature(fnbox)]
#![feature(unboxed_closures)]

extern crate alloc;

pub mod thread;
pub mod sync;
pub mod net;
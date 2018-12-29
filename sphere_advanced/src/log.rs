#![allow(non_camel_case_types)]
use core::marker::PhantomData;
use alloc::format;

type c_char = i8;

extern {
    fn Log_Debug(fmt: *const c_char) -> i32;
}

pub fn log_debug(fmt: &str){

    let null_ending = format!("{}\n\0", fmt);
    let cstr = CString::new(&null_ending);

    match cstr {
        Ok(data) => unsafe {
            Log_Debug(data.data);
        },
        _ => panic!()
    }


}

struct CString<'a> {
    data: *const c_char,
    len: usize,
    _marker: PhantomData<&'a c_char>
}

struct Error(&'static str);


fn ascii_guard(ptr: *const c_char, len: usize) -> bool {
    let mut ctr = 0usize;
    while ctr < len {
        if unsafe { *ptr.offset(ctr as isize) < 0 as c_char } {
            return false;
        }

        ctr += 1;
    }

    true
}


impl<'a> CString<'a> {
    pub fn new(s: &'a str) -> Result<CString<'a>, Error> {
        if s.len() == 0 {
            return Err(Error("0-length cstring found"));
        }

        let ret = CString {
            data: s.as_ptr() as *const c_char,
            len: s.len(),
            _marker: PhantomData
        };

        if ! ascii_guard(ret.data, ret.len) {
            return Err(Error("Invalid character in string"));
        }

        if unsafe { *ret.into_raw().offset(ret.len as isize - 1) != 0 } {
            Err(Error("No NULL terminator present"))
        } else {
            Ok(ret)
        }
    }

    pub unsafe fn into_raw(&self) -> *const c_char {
        self.data
    }

}
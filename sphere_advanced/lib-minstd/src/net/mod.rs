#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]

use core::mem;

type c_int = i32;
type c_uint = u32;
type c_void = core::ffi::c_void;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;

type x = u8;

#[repr(C)]
pub struct sockaddr4_in {
    pub sin_family: u16,
    pub sin_port: u16,
    sin_addr: in4_addr,
    sin_zero: (
        x, x, x, x,
        x, x, x, x,
        x, x, x, x,
        x, x, x, x,
        x, x, x, x)
}

#[repr(C)]
pub struct in4_addr {
    s_addr: c_uint
}

#[repr(C)]
pub enum sockaddr {
    ipv4(sockaddr4_in)
}

static AF_INET: i32 = 2_i32;
static SOCK_STREAM: i32 = 1_i32;
static INADDR_ANY: u32 = 0_u32;

extern {
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;

    fn bind(socket: c_int, address: *const sockaddr4_in, address_len: socklen_t) -> c_int;

    fn shutdown(socket: c_int, how: c_int) -> c_int;

    fn listen(socket: c_int, backlog: c_int) -> c_int;

    fn accept(socket: c_int, address: *mut sockaddr4_in,
                  address_len: *mut socklen_t) -> c_int;

    fn write(fd: c_int, buf: *const c_void, count: size_t)
                 -> ssize_t;

    fn read(fd: c_int, buf: *mut c_void, count: size_t)
                -> ssize_t;

    pub fn htons(hostshort: u16) -> u16;

    fn htonl(hostlong: u32) -> u32;
}

pub struct Socket {
    fd: c_int
}

impl Socket {
    pub fn write(&self, data: &[u8]) {
        unsafe {
            write(self.fd, data.as_ptr() as *const c_void, data.len());
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> isize {
        let read_count = unsafe { read(self.fd, buffer.as_mut_ptr() as *mut _, buffer.len()) };
        read_count
    }

    pub fn shutdown(&self) {
        unsafe {
            shutdown(self.fd, 2);
        }
    }
}

pub struct ServerSocket {
    fd: c_int
}

impl ServerSocket {
    pub fn new(port: u16) -> ServerSocket {

        let socket_fd = unsafe { socket(AF_INET, SOCK_STREAM, 0) };

        unsafe {
            let mut serv_address: sockaddr4_in = mem::zeroed();
            let addr_len = mem::size_of::<sockaddr4_in>();

            serv_address.sin_family = 2; // = AF_INET;
            serv_address.sin_addr.s_addr = htonl(INADDR_ANY);
            serv_address.sin_port = htons(port);

            bind(socket_fd, &mut serv_address, addr_len as u32);

            listen(socket_fd, 10);

            ServerSocket{
                fd: socket_fd
            }
        }
    }

    pub fn accept(&self) -> Socket {
        let connfd = unsafe { accept(self.fd, core::ptr::null_mut(), core::ptr::null_mut()) };

        Socket {
            fd: connfd
        }
    }

}
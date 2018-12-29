#![feature(lang_items, start, libc)]
#![feature(allocator_api)]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_std]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::string::String;
use alloc::string::ToString;
use alloc::format;

use alloc::sync::Arc;

mod mt3620_gpio;
#[cfg(not(test))]
mod allocator;
mod log;

use lowstd::thread;
use lowstd::sync;
use lowstd::net::ServerSocket;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL: allocator::MyAllocator = allocator::MyAllocator;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {

    log::log_debug("Initialize");
    let gpio = Arc::new(mt3620_gpio::Gpio::init());
    let mutex = Arc::new(sync::Mutex::new());

    loop {
        if !mt3620_gpio::is_networking_ready() {
            mt3620_gpio::sleep_seconds(1);
        } else {
            break;
        }
    }

    let ssocket = ServerSocket::new(5000);
    loop {
        let socket = ssocket.accept();
        let gpio = gpio.clone();
        let mutex = mutex.clone();

        thread::Thread::spawn(move || {
            log::log_debug("Lets blink!");

            socket.write("hello\r\n".as_bytes());

            let mut disconnected = false;
            while !disconnected {
                let mut line = String::new();
                let mut found_nl = false;

                while !found_nl {
                    let mut buffer = [0u8; 1024];
                    let count = socket.read(&mut buffer) as usize;

                    if count == 0 {
                        disconnected = true;
                        break;
                    }

                    if buffer[count - 1] == 10 {
                        found_nl = true;
                    }

                    let buffer = core::str::from_utf8(&buffer[0..count]).expect("!!!");
                    line.push_str(buffer);
                }

                line = line.trim().to_string();

                if !(line == "exit") {
                    let (error, led, r, g, b) = parse_line(line);

                    if error {
                        socket.write("error\r\n".as_bytes());
                    } else {
                        let answer = format!("set {} {} {} {}\r\n", led, r, g, b);
                        socket.write(answer.as_bytes());

                        let which_led = match led {
                            1 => mt3620_gpio::LED_1,
                            2 => mt3620_gpio::LED_2,
                            3 => mt3620_gpio::LED_3,
                            4 => mt3620_gpio::LED_4,
                            _ => mt3620_gpio::LED_1
                        };

                        mutex.lock();
                        if r {
                            gpio.set_low(which_led, mt3620_gpio::LED_COLOR_RED);
                        } else {
                            gpio.set_high(which_led, mt3620_gpio::LED_COLOR_RED);
                        }

                        if g {
                            gpio.set_low(which_led, mt3620_gpio::LED_COLOR_GREEN);
                        } else {
                            gpio.set_high(which_led, mt3620_gpio::LED_COLOR_GREEN);
                        }

                        if b {
                            gpio.set_low(which_led, mt3620_gpio::LED_COLOR_BLUE);
                        } else {
                            gpio.set_high(which_led, mt3620_gpio::LED_COLOR_BLUE);
                        }
                        mutex.unlock();
                    }
                } else {
                    socket.write("bye\r\n".as_bytes());
                    break;
                }
            }

            socket.shutdown();
        });
    }

    #[allow(unreachable_code)]
    0
}

fn parse_line(line: String) -> (bool, u32, bool, bool, bool) {
    let mut error = false;
    let mut led = 0;
    let mut r = false;
    let mut g = false;
    let mut b = false;

    if line.len() != 4 {
        error = true;
    } else {
        let mut chars = line.chars();

        let convert_led = chars.nth(0).unwrap().to_digit(10);

        match convert_led {
            Some(x) => led = x,
            _ => error = true
        }

        r = match chars.nth(0).unwrap() {
            '0' => false,
            '1' => true,
            _ => {
                error = true;
                false
            },
        };

        g = match chars.nth(0).unwrap() {
            '0' => false,
            '1' => true,
            _ => {
                error = true;
                false
            },
        };

        b = match chars.nth(0).unwrap() {
            '0' => false,
            '1' => true,
            _ => {
                error = true;
                false
            },
        };

        if led < 1 || led > 4 {
            error = true;
        }
    }

    return (error, led, r, g, b)
}

#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    log::log_debug("panic!");
    loop {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line_1() {
        assert_eq!(parse_line("1000".to_string()), (false, 1, false, false, false));
    }

    #[test]
    fn test_parse_line_2() {
        assert_eq!(parse_line("2100".to_string()), (false, 2, true, false, false));
    }

    #[test]
    fn test_parse_line_3() {
        assert_eq!(parse_line("3010".to_string()), (false, 3, false, true, false));
    }

    #[test]
    fn test_parse_line_4() {
        assert_eq!(parse_line("4001".to_string()), (false, 4, false, false, true));
    }

    #[test]
    fn test_parse_line_error_1() {
        assert_eq!(parse_line("5001".to_string()), (true, 5, false, false, true));
    }

    #[test]
    fn test_parse_line_error_2() {
        assert_eq!(parse_line("none_sense".to_string()), (true, 0, false, false, false));
    }
}
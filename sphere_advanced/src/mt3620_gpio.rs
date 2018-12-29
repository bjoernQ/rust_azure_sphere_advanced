use core::cell::RefCell;

extern {
    fn GPIO_OpenAsOutput(gpio_id: i32, output_mode: i8, value_type: i8) -> i32;
    fn GPIO_SetValue(fd: i32, value_type: i8) -> i32;
    fn sleep(seconds: u32) -> ();
    fn Networking_IsNetworkingReady(is_ready: &mut bool) -> i32;
}

pub const MT3620_GPIO8: i32 = 8;
pub const MT3620_GPIO9: i32 = 9;
pub const MT3620_GPIO10: i32 = 10;
pub const MT3620_GPIO15: i32 = 15;
pub const MT3620_GPIO16: i32 = 16;
pub const MT3620_GPIO17: i32 = 17;
pub const MT3620_GPIO18: i32 = 18;
pub const MT3620_GPIO19: i32 = 19;
pub const MT3620_GPIO20: i32 = 20;
pub const MT3620_GPIO21: i32 = 21;
pub const MT3620_GPIO22: i32 = 22;
pub const MT3620_GPIO23: i32 = 23;

pub const MT3620_RDB_LED1_RED: i32 = MT3620_GPIO8;
pub const MT3620_RDB_LED1_GREEN: i32 = MT3620_GPIO9;
pub const MT3620_RDB_LED1_BLUE: i32 = MT3620_GPIO10;

pub const MT3620_RDB_LED2_RED: i32 = MT3620_GPIO15;
pub const MT3620_RDB_LED2_GREEN: i32 = MT3620_GPIO16;
pub const MT3620_RDB_LED2_BLUE: i32 = MT3620_GPIO17;

pub const MT3620_RDB_LED3_RED: i32 = MT3620_GPIO18;
pub const MT3620_RDB_LED3_GREEN: i32 = MT3620_GPIO19;
pub const MT3620_RDB_LED3_BLUE: i32 = MT3620_GPIO20;

pub const MT3620_RDB_LED4_RED: i32 = MT3620_GPIO21;
pub const MT3620_RDB_LED4_GREEN: i32 = MT3620_GPIO22;
pub const MT3620_RDB_LED4_BLUE: i32 = MT3620_GPIO23;

pub const GPIO_OUTPUT_MODE_PUSH_PULL: i8 = 0;
pub const GPIO_VALUE_LOW: i8 = 0;
pub const GPIO_VALUE_HIGH: i8 = 1;

pub type LedNumber = i8;
pub type LedColor = i8;

pub const LED_1: LedNumber = 0;
pub const LED_2: LedNumber = 1;
pub const LED_3: LedNumber = 2;
pub const LED_4: LedNumber = 3;

pub const LED_COLOR_GREEN: LedColor = 0;
pub const LED_COLOR_BLUE: LedColor = 1;
pub const LED_COLOR_RED: LedColor = 2;

pub struct Gpio {
    fd: RefCell<[[i32; 3] ; 4 ]>,
    fd_open: RefCell<[[bool; 3] ; 4 ]>
}

impl Gpio {
    pub fn init() -> Gpio {
        Gpio {
            fd: RefCell::new([[0; 3] ; 4 ]),
            fd_open: RefCell::new([[false; 3] ; 4 ])
        }
    }

    pub fn set_high(&self, led: LedNumber, color: LedColor){
        self.ensure_open(led, color);
        unsafe { GPIO_SetValue(self.fd.borrow()[led as usize][color as usize], GPIO_VALUE_HIGH); }
    }

    pub fn set_low(&self, led: LedNumber, color: LedColor){
        self.ensure_open(led, color);
        unsafe { GPIO_SetValue(self.fd.borrow()[led as usize][color as usize], GPIO_VALUE_LOW); }
    }

    fn ensure_open(&self, led: LedNumber, color: LedColor){
        if !self.fd_open.borrow()[led as usize][color as usize] {
            self.fd_open.borrow_mut()[led as usize][color as usize] = true;
            let mt3620_led = match led {
                LED_1 => match color {
                     LED_COLOR_GREEN => MT3620_RDB_LED1_GREEN,
                     LED_COLOR_BLUE => MT3620_RDB_LED1_BLUE,
                     LED_COLOR_RED => MT3620_RDB_LED1_RED,
                     _ => panic!()
                },
                LED_2 => match color {
                    LED_COLOR_GREEN => MT3620_RDB_LED2_GREEN,
                    LED_COLOR_BLUE => MT3620_RDB_LED2_BLUE,
                    LED_COLOR_RED => MT3620_RDB_LED2_RED,
                    _ => panic!()
                },
                LED_3 => match color {
                    LED_COLOR_GREEN => MT3620_RDB_LED3_GREEN,
                    LED_COLOR_BLUE => MT3620_RDB_LED3_BLUE,
                    LED_COLOR_RED => MT3620_RDB_LED3_RED,
                    _ => panic!()
                },
                LED_4 => match color {
                    LED_COLOR_GREEN => MT3620_RDB_LED4_GREEN,
                    LED_COLOR_BLUE => MT3620_RDB_LED4_BLUE,
                    LED_COLOR_RED => MT3620_RDB_LED4_RED,
                    _ => panic!()
                },
                _ => panic!()
            };

            self.fd.borrow_mut()[led as usize][color as usize] = unsafe {
                GPIO_OpenAsOutput(mt3620_led, GPIO_OUTPUT_MODE_PUSH_PULL, GPIO_VALUE_HIGH)
            }
        }
    }
}

pub fn sleep_seconds(seconds: u32) {
    unsafe {
        sleep(seconds);
    }
}

pub fn is_networking_ready() -> bool {
    let mut is_ready = false;

    unsafe {
        Networking_IsNetworkingReady(&mut is_ready);
    };

    is_ready
}
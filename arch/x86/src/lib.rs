#![no_std]
#![feature(global_asm)]

pub mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;
pub mod x86lib;

pub const ARCH: &'static str = "x86";

pub static SLEEP_TIME: i32 = 10000000;

// include/src/time.rs
fn sleep(seconds: i32) {
    let mut x = 0;
    let time = seconds * 100000000;
    loop {
        x = x + 1;
        if x == time {
            break;
        }
    }
}

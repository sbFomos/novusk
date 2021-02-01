#![macro_use]

use crate::x86::lib::print::WRITER;
use core::fmt::{Arguments, Write};
use crate::x86::kernel::time::time::kernel_time;

pub fn _e_kinfo(args: Arguments) {
    write!(WRITER.lock(), "[ {} ] {}", kernel_time(), args);
}

#[macro_export]
macro_rules! e_kinfo {
    ($($arg:tt)*) => {$crate::x86::kernel::early_info::_e_kinfo(format_args!($($arg)*))};
}
#![no_std]

extern crate arch;
use arch::x86::kernel::vga_buffer::Color::*;
extern crate include;
use include::{time::{sleep}};

#[macro_use]
extern crate novusk_lib;
use novusk_lib::ui;

pub fn main() {
    println!("Starting example os...");
    sleep(1);
    ui::display::fill_screen(Cyan);
}

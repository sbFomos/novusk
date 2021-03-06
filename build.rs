use std::fs::{File, write};
use std::process::{Command};

extern crate serde_json;
use serde_json::{from_reader, Value};

fn write_space(file: &str) -> std::io::Result<()> {
    write(file, "\n")
}

const BOARD_NAME: &str = "arch/aarch64/src/boot/name.rs";
const UART_DRIVER: &str = "arch/aarch64/src/drivers/uart.rs";

fn setup_virt() {
    write(UART_DRIVER, "pub const: UART0: *mut u8 = 0x0900_0000 as *mut u8;");
}

fn build_aarch64() {
    /* ----- Read JSON file ----- */
    let defconfig_file = File::open("arch/aarch64/src/configs/defconfig.json").unwrap();
    let defconfig: Value = from_reader(defconfig_file)
        .expect("Couldn't read json from defconfig file");
    let board = defconfig.get("board")
        .expect("Couldn't read board name from defconfig");
    if board == "virt" {
        setup_virt();
    } else { setup_virt(); }
}

const VGA_DRIVER: &str = "arch/src/x86/drivers/vga_text.rs";

fn setup_screen_200() {
    write(VGA_DRIVER, "pub const BUFFER_HEIGHT: usize = 25; pub const BUFFER_WIDTH: usize = 80; pub const SCREEN_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH; pub const VGA_BUFFER: i32 = 0xb8000;");
}

fn setup_screen_6400() {
    write(VGA_DRIVER, "pub const BUFFER_HEIGHT: usize = 200; pub const BUFFER_WIDTH: usize = 320; pub const SCREEN_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH; pub const VGA_BUFFER: i32 = 0xa0000;");
}

fn build_x86() {
    /* ----- Read JSON file ----- */
    let defconfig_file = File::open("arch/x86/src/configs/defconfig.json").unwrap();
    let defconfig: Value = from_reader(defconfig_file)
        .expect("Couldn't read json from defconfig file");
    /* ----- ----- */
    let screen_size = defconfig.get("screen_size")
        .expect("Couldn't read screen_size value from defconfig");
    if screen_size == "200" {
        setup_screen_200();
    } else if screen_size == "6400" {
        setup_screen_6400();
    } else { setup_screen_200(); }
}

fn main() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    build_x86();

    #[cfg(any(target_arch = "aarch64"))]
    build_aarch64();
}

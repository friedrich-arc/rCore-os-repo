//! 在 U 模式（用户态）执行 S 模式的特权指令 `sret`，
//! 应当触发“非法指令”异常，被内核杀死。

#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Try to execute privileged instruction in U Mode");
    println!("Kernel should kill this application!");
    unsafe {
        asm!("sret");
    }
    0
}//非法原因：使用了sret指令,（cpu认识这个指令是s级特权指令，用户态无法执行，cpu会报警，交给内核处理）该指令只能在内核态跑，属于越界，会被内核杀死

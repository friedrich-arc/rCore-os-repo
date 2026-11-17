//! 在 U 模式（用户态）访问 S 模式专属的 CSR（sstatus），
//! 应当触发“非法指令”异常，被内核杀死。
//!
//! 教程里这一句写作：
//! ```ignore
//! use riscv::register::sstatus::{self, SPP};
//! unsafe { sstatus::set_spp(SPP::User); }
//! ```
//! 那需要额外引入 `riscv` 这个外部 crate。为了让本章的用户库保持“零依赖”、
//! 不因为下载依赖而卡住，这里用等价的内嵌汇编直接写 sstatus 寄存器：
//! 在 U 模式下执行任何针对 sstatus 的 csr 指令都会立刻触发非法指令异常，
//! 与 `set_spp` 的效果完全相同（异常类型都是 IllegalInstruction）。

#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Try to access privileged CSR in U Mode");
    println!("Kernel should kill this application!");
    unsafe {
        // csrw sstatus, a0 —— 往 sstatus（S 模式 CSR）写 0
        asm!("csrw sstatus, {}", in(reg) 0usize);
    }
    0
}//越界访问s级寄存器，被内核杀死

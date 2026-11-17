//! 用户库（第二章）：所有用户程序共用的“标准库”。
//! 提供三样东西：
//! 1. 程序入口 `_start`（链接脚本把它放在 0x80400000 的最开头）；
//! 2. `println!` 输出能力（底层用 `ecall` 发起 sys_write）；
//! 3. `exit` 退出接口（底层是 sys_exit）。
//!
//! 每个应用只需要在 `src/bin/*.rs` 里提供 `main` 函数，
//! `_start` 清空 `.bss` 之后会调用它。

#![no_std]

#[macro_use]
pub mod console;//pub → 对外公开，其他 crate（如 bin/*.rs）能通过 user_lib::console::print() 访问，公开给所有依赖 user_lib 的外部 crate（即user下所有crate）
mod lang_items;
mod syscall;

use syscall::*;//把 syscall 模块里所有 pub 标记的东西，一次性拉进当前作用域，后面写代码时不用再前缀 syscall::。

/// 链接脚本里定义的符号，用于清空 .bss 段
macro_rules! linker_symbol_addr {
    ($symbol:path) => {
        ($symbol as *const ()).addr()
    };
}//把链接脚本里定义的符号转变为安全的内存地址（usize),因为链接脚本里写的符号不是变量，没有存储空间，只是一个"地址标签"，同内核
/*整个宏的含义就是：把链接脚本里定义的符号，变成 Rust 里能用的地址数字（usize）。 */
unsafe extern "Rust" {
    fn main() -> i32;
}//告知编译器，这个名为main的函数不在此文件内，他没有参数，返回一个i32，
//但是现在不告知编译器这个函数长什么样，具体代码是什么，等把所有文件链接在一起后，自然会有一个文件把它实现

/// 用户程序的真正入口。
//#[unsafe(link_section = ".text.entry")]` 把这段代码放进 `.text.entry` 段，
#[unsafe(no_mangle)]  //#[unsafe(no_mangle)]` 保证符号名不被改写，链接脚本才能按名字找到它。
#[unsafe(link_section = ".text.entry")]/// 链接脚本会把它排在程序最前面，于是内核跳到 0x80400000 就正好进入这里。
pub extern "C" fn _start() -> ! {
    clear_bss();
    unsafe {
        // 调用 bin 里的 main，并把返回值通过 sys_exit 告诉内核
        exit(main());
    }
    panic!("unreachable after sys_exit!");
}
/// `start_bss` / `end_bss` 来自 `src/linker.ld`。
fn clear_bss() {
    unsafe extern "C" {
        safe fn start_bss();
        safe fn end_bss();
    }
    (linker_symbol_addr!(start_bss)..linker_symbol_addr!(end_bss)).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}//栈清空

/// 向文件描述符 `fd` 写入缓冲区 `buf`，返回实际写入的长度。
pub fn write(fd: usize, buf: &[u8]) -> isize {//fd是文件描述符，1代表标准输出，0标准输入，2标准错误
    sys_write(fd, buf)
}

/// 退出当前应用程序，并把返回值告知批处理系统。
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}//exit_code是退出码，0代表正常退出，非0则是异常退出

#![no_std]//不使用标准库//
#![no_main]//不使用普通的main入口//
#[macro_use]
mod sbi;
mod console;
mod lang_items;//开机后cpu执行的第一条指令（risc-v汇编），引入lang——items里的代码//
use core::arch::global_asm;//从rust核心库里体系模块中引入内联汇编宏，这样我下面可以直接用global_asm!()，将汇编字符串转换为risc-v的机器指令//
global_asm!(include_str!("entry.asm"));/*干净、清晰、好维护，因此分开写,作用：将entry.asm文件中的代码作为字符串引入，再通过global_asm!把它写入内核运行。*/


macro_rules! linker_symbol_addr { /*定义一个声明宏（declarative macro），名字叫 linker_symbol_addr。 */
                                /*macro_rules! 是 Rust 里写宏的关键字。这个宏的作用：它接收一个符号名，返回那个符号的地址值。 */
    ($symbol:path) => {         /*这是匹配规则。意思是：这个宏只接受一种输入——一个 $symbol:path：linker_symbol_addr!(sbss) path linker_symbol_addr!(ebss)  */ 
        ($symbol as *const ()).addr()  /*as *const () 是强制类型转换（cast），把 sbss 这个符号转成一个"指向空类型的裸指针"。 */
    };
}/*整个宏的含义就是：把链接脚本里定义的 sbss 和 ebss 这两个符号，变成 Rust 里能用的地址数字（usize）。 */

#[unsafe(no_mangle)]/*#[no_mangle]：禁止编译器修改函数名，确保汇编里的 call rust_main 能精准找到它。*/ 
pub fn rust_main() -> ! {
    clear_bss();/*之前的汇编里定义了 .bss.stack（64KB栈空间）和 .bss 段。虽然汇编预留了空间，但.bss 段（未初始化全局变量）在内存中必须被清零，否则里面是随机垃圾值。 */  
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
/*CPU 上电 -> 跳到 _start（汇编）
汇编设置 sp 指向 boot_stack_top（64KB 栈就绪）
汇编 call rust_main -> 进入 Rust
rust_main 执行 loop {}（系统挂起，无溢出风险*/
fn clear_bss() {
    unsafe extern "C" { /*sbss等符号遵循 C ABI（ */
        safe fn sbss();/*声明两个"函数"：名字叫 sbss / ebss它们没有参数，没有返回值 */
        safe fn ebss();
    }
    (linker_symbol_addr!(sbss)..linker_symbol_addr!(ebss)).for_each(|a| {/* (linker_symbol_addr!(sbss)..linker_symbol_addr!(ebss))指从sbss到ebss的范围 */
                                                                                /*.for_each(|a|是指：对迭代器产出的每一个元素，执行闭包里的代码。 */
        unsafe { (a as *mut u8).write_volatile(0) }/*对每个地址 a，做两件事：把 usize（rust里的一种整数类型） 地址转成"指向 u8 的可变裸指针"。然后往这个指针指向的地址强制写入 0。*/
    });
}/*整个函数作用：从 sbss 到 ebss，每个字节写 0。即清空栈内未初始化的内容 */




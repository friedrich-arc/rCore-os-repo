//! `#![no_std]` 环境下必须自己提供的 lang item：panic 处理函数。
//!
//! 用户程序 panic 时先打印位置信息，再调用 sys_exit(-1) 退出，
//! 不会让整个系统卡死在一个死循环里。

use core::panic::PanicInfo;//引入PanicInfo这个结构体，当程序发生严重和错误时，系统将错误信息打包入结构体，传给其中的处理函数//

use crate::exit;//从根目录lib.rs开始找exit//

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {//if let = "如果模式匹配成功，就把匹配到的值绑定给变量，然后进这个分支。//
        println!(
            "Panicked at {}:{}, {}", //如果 panic_info.location() 返回的是 Some(位置信息)，就把里面的位置信息"解"出来赋给 location，然后进入 if 分支，location 在分支里直接用。
            location.file(),//返回 panic 发生的源文件名
            location.line(),//返回 panic 发生的行号
            panic_info.message()//返回 panic 时附带的格式化消息：**返回什么取决于 panic 怎么触发的
        );
    } else {
        println!("Panicked: {}", panic_info.message());
    }
    exit(-1);//告诉内核："我 panic 了，退出码 -1"
    loop {}
}//恐慌处理函数

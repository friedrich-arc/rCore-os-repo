#![no_std]
#![no_main]

#[macro_use]//把 user_lib 里所有标记了 #[macro_export] 的宏自动导入到当前文件的宏命名空间//
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("Hello, world from user mode program!");
    0//返回值，i32数据类型，表正常退出//
}
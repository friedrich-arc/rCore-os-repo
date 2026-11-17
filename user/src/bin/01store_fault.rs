#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("Into Test store_fault, we will insert an invalid store operation...");
    println!("Kernel should kill this application!");
     // 尝试向物理地址 0 写入数据，这绝对是非法的！👇，因为在现代操作系统中，0地址都是被严格保留的
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0); // 使用 core::ptr::null_mut() 获取空指针，然后写入 0：解析（禁用std标准库，用core：：ptr申请裸指针创建可变空指针，类型是u8，值是0)
    }   
    0
}
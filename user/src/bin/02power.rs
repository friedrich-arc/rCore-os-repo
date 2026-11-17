#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const SIZE: usize = 10;
const P: u32 = 3;
const STEP: usize = 100000;
const MOD: u32 = 10007;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut pow = [0u32; SIZE];// 10 个元素的数组，初始全 0，环形
    let mut index: usize = 0;
    
    pow[index] = 1;
    for i in 1..=STEP {//无上限
        let last = pow[index];
        index = (index + 1) % SIZE;
        pow[index] = last * P % MOD;
        
        // 每计算 10000 次，打印一次状态
        if i % 10000 == 0 {
            println!("{}^{}={}", P, i, pow[index]);
        }
    }
    println!("Test power OK!");
    0
}
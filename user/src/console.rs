use crate::write;//从根目录lib.rs开始找write//
use core::fmt::{self, Write};

const STDOUT: usize = 1;//定义一个常量 STDOUT，值 = 1,Unix 约定：文件描述符 1 = 标准输出,后面调 write(STDOUT, ...) 就是"往屏幕写"//

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());// s.as_bytes()把&str 转成 &[u8]//
        Ok(())//表示写入成功//
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}//同内核态的print，将一段一段的字符串起来丢到串口出输出

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
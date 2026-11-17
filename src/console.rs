/*掌管打印输出字符 ：格式化输出字符：封装print！和println！宏，支持复杂化格式输出*/
use crate::sbi::console_putchar;/*引入sbi函数 */
use core::fmt::{self, Write};/* 引入格式化打印模块core:fmt和write，保证有write_str功能通过接口*/

struct Stdout;/*定义空结构体，代表串口终端 */

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {/*mut可变引用 */
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}/*真正证明stdout拥有能够write_str权力，该函数：它把传进来的字符串，一个字符一个字符地送到串口去。 */

pub fn print(args: fmt::Arguments) {/*args: fmt::Arguments：这是已经处理好的格式化参数，但不是最后要的发送给串口的字符串，还要接下来write_fmt处理 */
    Stdout.write_fmt(args).unwrap();/*write_fmt内部多次调用write_str处理args，write_fmt 内部调用 write_str 时，每次传的就是一段 &str，write_str 拿到这段 &str 后，一个字符一个字符丢给串口 */
}

#[macro_export]/*让全局都可以用 */
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}/*输出但不换行 */

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}/*	输出并换行 */
/*这两个宏的作用：这两个宏就是把 print!("x = {}", 10) 这种写法，自动翻译成 console::print(format_args!("x = {}", 10))。 */
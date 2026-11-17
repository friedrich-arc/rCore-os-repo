//与底层固件通信：通过ecall向M模式的rustsbi请求服务//

pub fn console_putchar(c: usize) { /*采用usize输入，适配寄存器而非限定在某一平台，因此不要限定位数 */
    #[allow(deprecated)]/*使用已经弃用的接口，阻止报警*/
    sbi_rt::legacy::console_putchar(c);/*调用库的函数：打印出c指代的字符内容：比如输入65或者‘A’as usize，打印出A */
}
/*我们写的 console_putchar 和 sbi_rt::legacy::console_putchar 是两个完全不同的函数，只是名字碰巧都叫 console_putchar。 */
/*不是原样输出usize数字，而是而是"把这个数字当 ASCII 码，输出对应的字符"*/
/*整个函数的作用就是：输入数字，再输出数字对应的ASCCI 对应的字符 */
pub fn shutdown(failure: bool) -> ! {/*接收布尔值，且永远不返回*/
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};/*引用sbi_rt库提供的服务:system_reset系统复位，Shutdown关机，SystemFailure告知关机原因 */
    if !failure {/*如果接收到true代表内核遇到不可恢复的错误， */
        system_reset(Shutdown, NoReason);
    } else {/*接收到false代表正常关机*/
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}  /*告知编译器如果跑到了这里代表有重大错误发生，可以直接报错 */
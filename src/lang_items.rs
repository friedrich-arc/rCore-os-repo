//语言项补全//
use crate::println;
use crate::sbi::shutdown;
use core::panic::PanicInfo;//引入PanicInfo这个结构体，当程序发生严重和错误时，系统将错误信息打包入结构体，传给其中的处理函数//
//程序恐慌处理函数👇//
#[panic_handler]//rust属性写法：#[].将该程序登记为全程序的恐慌处理程序，即一旦发生严重错误交给这个程序来处理，一个 crate 只能有一个​ #[panic_handler]//
fn panic(info: &PanicInfo) -> ! {//“->”代表这个函数一旦被调用就永远不可能返回//
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    shutdown(true)
}
/*这段代码的唯一目的：当内核发生 panic（致命错误）时，打印出错信息，然后关机。 */
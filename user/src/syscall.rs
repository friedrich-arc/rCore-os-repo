use core::arch::asm;//引入内联汇编

const SYSCALL_WRITE: usize = 64;//约定符号。告知内核打印
const SYSCALL_EXIT: usize = 93;//约定符号，告知内核退出

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;   
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,//存放参数并且返回值也存放在这里
            in("x11") args[1],//存放参数
            in("x12") args[2],//存放参数
            in("x17") id//存放系统调用号
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])//fd：文件描述符，buffer.as_ptr()代表文件首地址， buffer.len()文件数据长度
}//接口

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
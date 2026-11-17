OUTPUT_ARCH(riscv)
ENTRY(_start)

/* 四个用户程序都会被内核加载到这个固定物理地址上 */
BASE_ADDRESS = 0x80400000;    /*开始地址记为0x80400000*/

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)          /* _start 必须排在最前面 */
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss : {
        start_bss = .;          /* clear_bss() 用到的起始地址 */
        *(.bss .bss.*)             /*无stack分配：因为用户栈由内核分配*/
        *(.sbss .sbss.*)
        end_bss = .;            /* clear_bss() 用到的结束地址 */
    }
    /DISCARD/ : {/*“扔掉没用的数据，防止布局错位”。*/
        *(.eh_frame)
        *(.eh_frame_hdr)
        /* 纯元数据段：不参与运行，且新版 rust-lld 会把它们摆到
           与 .text / .rodata 重叠的文件偏移上，这里直接丢弃。 */
        *(.comment)
        *(.note*)
        *(.riscv.attributes)
        *(.debug*)
    }
}

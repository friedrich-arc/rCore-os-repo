# 汇编启动入口：CPU执行的正式指令的第一条指令所在地
.section .text.entry        # 告知链接器将接下来的代码放入 .text.entry 里
.globl _start               # 定义一个全局符号，所有文件都可以找到它
_start:                     # 标签：每当有 _start 出现就指向这里
    la sp, boot_stack_top   # 就是把栈顶的地址存入sp寄存器
    call rust_main          # call = jal ra, rust_main，此处ra会被设为_start的下一条指令的地址

.section .bss.stack         #设置栈空间
.globl boot_stack_lower_bound #定义全局标签
boot_stack_lower_bound:       #低地址
    .space 4096 * 16          #留出空间
    .globl boot_stack_top     #定义全局标签     
boot_stack_top: #从 boot_stack_lower_bound 这个地址开始，往后留 64KB 空地，然后给这块空地的末尾贴个标签叫 boot_stack_top
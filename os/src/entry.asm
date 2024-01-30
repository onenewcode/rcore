     .section .text.entry //声明后面的内容全部放在.text.entry的段中
     .globl _start //声明_start是一个全局符号
_start:
    la sp, boot_stack_top //加载栈指针
    call rust_main //把程序权限交给rust中的rust_main函数

    .section .bss.stack
    .globl boot_stack_lower_bound //声明栈底
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top //声明栈顶
boot_stack_top: 
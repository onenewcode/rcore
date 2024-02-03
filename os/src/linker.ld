OUTPUT_ARCH(riscv) /*设置编译的目标平台*/
ENTRY(_start) /*设置整个程序的入口*/
BASE_ADDRESS = 0x80200000;/*设置全局变量就，作为程序的起始地址*/

SECTIONS  /*链接过程中对输入的目标文件的段的合并*/
{/*.表示当前地址*/
    . = BASE_ADDRESS;
    skernel = .;
    stext = .;
    /* .text 代码段，包含已编译的目标文件中的机器码。 */
    .text : { /*冒号前面表示最终生成的可执行文件的一个段的名字，花括号内按照放置顺序描述将所有输入目标文件的哪些段放在这个段中*/
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;
    srodata = .;
    /* rodata 只读数据段，存放程序中不会被修改的数据（如常量、字符串表等） */
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;
    /* .data：初始化过的数据段，这些数据在程序运行前需要被载入到内存中，并且在运行过程中可能被修改。  */
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    /* .bss：未初始化数据段，其中的数据在程序启动时会被清零，用于存储全局或静态未初始化变量。 */
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
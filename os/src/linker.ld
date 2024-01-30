OUTPUT_ARCH(riscv) /*设置编译的目标平台*/
ENTRY(_start) /*设置整个程序的入口*/
BASE_ADDRESS = 0x80200000;/*设置全局变量就，作为程序的起始地址*/

SECTIONS  /*链接过程中对输入的目标文件的段的合并*/
{/*.表示当前地址*/
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : { /*冒号前面表示最终生成的可执行文件的一个段的名字，花括号内按照放置顺序描述将所有输入目标文件的哪些段放在这个段中*/
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
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
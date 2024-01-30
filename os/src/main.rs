#![no_main] //声明没有主函数
#![no_std] //声明基于非标准库
#![feature(panic_info_message)]//开启anic_info_message属性
#[macro_use]
mod lang_items;
use core::arch::global_asm;
mod console;
mod sbi;

global_asm!(include_str!("entry.asm"));
#[no_mangle] //#[no_mangle]以避免编译器对它的名字进行混淆
pub fn rust_main()-> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
// 将.bss段的信息归0
fn clear_bss() {
    extern "C" {
        fn sbss(); //.bss起始地址
        fn ebss(); //.bss终止地址
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

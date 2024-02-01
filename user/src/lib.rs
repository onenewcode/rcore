#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
mod lang_items;
pub mod console;
mod syscall;
use syscall::*; 
#[no_mangle]
#[link_section = ".text.entry"]//的作用是在编译链接阶段指示编译器或链接器将特定的函数或者数据放置在最终生成的可执行文件或库的特定节（section）中。
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}
#[linkage = "weak"]//我们使用 Rust 的宏将其函数符号 main 标志为弱链接。这样在最后链接的时候，虽然在 lib.rs 和 bin 目录下的某个应用程序都有 main 符号，但由于 lib.rs 中的 main 符号是弱链接，链接器会使用 bin 目录下的应用主逻辑作为 main 。
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
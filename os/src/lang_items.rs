use core::panic::PanicInfo;
use crate::sbi::shutdown;
use crate::println;
#[panic_handler] //标记核心库panic!宏要对接的函数
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}
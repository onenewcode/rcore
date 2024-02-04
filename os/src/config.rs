//! Constants used in rCore

pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
//个条件编译属性。它告诉 Rust 编译器仅当项目启用了名为 "board_qemu" 的特性时，才编译并包含下面的常量定义。
#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;
// 这行代码引入了同一crate（库）内部的模块 board 中的 CLOCK_FREQ 常量，并将其作为当前作用域内的公有成员。
pub use crate::board::CLOCK_FREQ;
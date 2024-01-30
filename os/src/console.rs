use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;//结构体 Stdout 不包含任何字段，因此它被称为类单元结构体

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export] // 用于标记一个宏定义可以被外部crate导入和使用。
macro_rules! print {
    // iteral表示匹配字面值
    ($fmt: literal $(, $($arg: tt)+)?) => {//tt 表示单棵标记树
            // 使用 format_args! 创建 Arguments 实例，但并不立即格式化字符串
            // 下面的代码调用15的print函数
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
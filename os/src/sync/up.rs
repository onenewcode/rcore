use core::cell::{RefCell, RefMut};
pub struct UPSafeCell<T> {
    inner: RefCell<T>,// Rust 语言中的一个智能指针类型，它提供了运行时借用检查机制，允许在不可变（immutable）或借用（borrowed）上下文中动态修改其内部值。
}

// 首先声明实现了 Sync 特征，这表示该类型在多线程环境是线程安全的。
unsafe impl<T> Sync for UPSafeCell<T> {}
// 
impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    ///调用此方法意味着在同一时间只有一个线程可以获取到这个可变引用，
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

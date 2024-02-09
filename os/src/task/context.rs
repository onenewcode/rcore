
/// Task Context
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    // return address ( e.g. __restore ) of __switch ASM function
    ra: usize,
    // kernel stack pointer of app
    sp: usize,
    // callee saved registers:  s 0..11
    s: [usize; 12],
}

impl TaskContext {
    // 因为现在是静态加载所以还需要初始化上下文
    // init task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0, //一个专门用来保存子程序或函数返回地址的寄存器
            sp: 0, //栈指针
            s: [0; 12],
        }
    }

    // set task context {__restore ASM funciton, kernel stack, s_0..12 }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
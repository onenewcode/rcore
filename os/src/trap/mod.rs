mod context;
pub use context::TrapContext;
use crate::batch::run_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    // __alltraps()内核态转换成用户态
    extern "C" { fn __alltraps(); }
    unsafe {
        // 在 RISC-V 系统中，当处理器检测到异常、中断或其他需要进入特权模式处理的事件时，
        //会触发陷阱（trap），并使用 stvec 寄存器指向的地址作为陷阱处理程序的入口地址。
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    // scause  描述 Trap 的原因
    let scause = scause::read();
    // stval  给出 Trap 附加信息
    let stval = stval::read();
    match scause.cause() {
        //系统调用，表示用户用ecall函数调用，
        Trap::Exception(Exception::UserEnvCall) => {
            //sepc  异常程序计数器
            cx.sepc += 4;//指向ecall指令的下一条地址,因为程序简单只需要跳过ecall所以地址加4
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}

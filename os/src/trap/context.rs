use riscv::register::sstatus::{self, Sstatus, SPP};
#[repr(C)] //用于指定结构体（struct）或枚举（enum）的内存布局应该与 C 语言中的相应类型保持一致。这意味着：
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus, //给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
    pub sepc: usize, //当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址
    pub kernel_satp: usize, //表示内核地址空间的 token ，即内核页表的起始物理地址；
    pub kernel_sp: usize, //表示当前应用在内核地址空间中的内核栈栈顶的虚拟地址；
    pub trap_handler: usize, //表示内核中 trap handler 入口点的虚拟地址。
}
// 在从操作系统内核返回到运行应用程序之前，要完成如下这些工作：
// 构造应用程序开始执行所需的 Trap 上下文；
// 通过 __restore 函数，从刚构造的 Trap 上下文中，恢复应用程序执行的部分寄存器；
// 设置 sepc CSR的内容为应用程序入口点 0x80400000；
// 切换 scratch 和 sp 寄存器，设置 sp 指向应用程序用户栈；
// 执行 sret 从 S 特权级切换到 U 特权级。

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize, //表示内核地址空间的 token ，即内核页表的起始物理地址；
        kernel_sp: usize, // 表示当前应用在内核地址空间中的内核栈栈顶的虚拟地址；
        trap_handler: usize, // 表示内核中 trap handler 入口点的虚拟地址。
    ) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,  // entry point of app
            kernel_satp,  // addr of page table
            kernel_sp,    // kernel stack
            trap_handler, // addr of trap_handler function
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
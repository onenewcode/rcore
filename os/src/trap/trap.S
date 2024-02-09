# 这行指令启用了一个名为.altmacro的汇编器扩展功能。在RISC-V GNU汇编器中，这个扩展允许使用更灵活的宏定义语法。
.altmacro
# 宏的作用是将寄存器 xn 中的内容保存到当前栈帧中，偏移地址为从栈顶向下数的第n*8字节处。这通常是在函数调用、任务切换或者保存上下文时，为了保护现场数据而进行的操作。
# 例如 SAVE_GP 10，宏将会展开为：
# sd x10, 10*8(sp)
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2 #确保下一条指令的地址是按照2的幂对齐的（通常意味着4字节对齐）。
__alltraps:
# csrrw rd, csr, rs1
# 首先，从 CSR 地址处读取当前值。
# 然后，将 rs1 寄存器中的值写入到指定的 CSR 中。
# 最后，将读取的 CSR 原始值存入目的寄存器 rd
    csrrw sp, sscratch, sp
# 现在是 sp->kernel 堆栈，sscratch->user 堆栈
# 在内核堆栈上分配一个 TrapContext    
# 减少栈指针sp的值。这里的操作是将栈顶向下移动34个8字节的空间来存储一些临时数据或上下文信息
    addi sp, sp, -34*8
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    .set n, 5
# 使用.rept循环和SAVE_GP宏，依次保存x5到x31寄存器的内容到内核栈。
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    # sp 的值复制到第一个函数参数寄存器 a0 中。这条指令常用于将当前的栈帧地址传递给一个函数作为参数。
    mv a0, sp
    call trap_handler
__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    # mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp) 
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # 恢复除 SP/TP 之外的 General-Purpuse 寄存器
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret
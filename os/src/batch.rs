const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

// 程序管理器
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

unsafe fn load_app(&self, app_id: usize) {
    if app_id >= self.num_app {
        panic!("All applications completed!");
    }
    println!("[kernel] Loading app_{}", app_id);
    // 清空app区域
    core::slice::from_raw_parts_mut(
        APP_BASE_ADDRESS as *mut u8,
        APP_SIZE_LIMIT
    ).fill(0);
    // 从内存中获取程序
    let app_src = core::slice::from_raw_parts(
        self.app_start[app_id] as *const u8,
        self.app_start[app_id + 1] - self.app_start[app_id]
    );
    let app_dst = core::slice::from_raw_parts_mut(
        APP_BASE_ADDRESS as *mut u8,
        app_src.len()
    );
    app_dst.copy_from_slice(app_src);
//    使用汇编指令 "fence.i" 来执行一个指令栅栏（Instruction Fence），保证之前的所有写操作都完成并刷新到内存中，
    asm!("fence.i");
}
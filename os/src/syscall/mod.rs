//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;

mod fs;
mod process;

use crate::task::add_current_syscall_count;
use fs::*;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    // 不认识的调用会先 panic 掉，不会进入到统计
    let handler: fn([usize; 3]) -> isize = match syscall_id {
        SYSCALL_WRITE => |args| sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => |args| sys_exit(args[0] as i32),
        SYSCALL_YIELD => |_| sys_yield(),
        SYSCALL_GET_TIME => |args| sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TRACE => |args| sys_trace(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    };

    // 同时能保证统计先于执行
    add_current_syscall_count(syscall_id);
    handler(args)
}

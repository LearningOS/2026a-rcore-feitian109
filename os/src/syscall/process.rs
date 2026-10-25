//! Process management syscalls
use crate::config::PAGE_SIZE;
use crate::mm::{translated_byte_buffer_safe, MapPermission};
use crate::task::{
    change_program_brk, current_user_token, exit_current_and_run_next, get_current_syscall_count,
    mmap_current, munmap_current, suspend_current_and_run_next,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let size = core::mem::size_of::<TimeVal>();
    // 转换为 u8 数组方便写入对应内存
    // SAFETY: TimeVal 字段布局稳定
    let src = unsafe { core::slice::from_raw_parts(&ts as *const TimeVal as *const u8, size) };
    let Some(buffers) =
        translated_byte_buffer_safe(current_user_token(), _ts as *const u8, size, true)
    else {
        return -1;
    };

    let mut i = 0;
    for buf in buffers {
        buf.copy_from_slice(&src[i..i + buf.len()]);
        i += buf.len();
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let token = current_user_token();

    match _trace_request {
        0 => {
            if let Some(buf) = translated_byte_buffer_safe(token, _id as *const u8, 1, false) {
                return buf[0][0] as isize;
            }
            -1
        }
        1 => {
            if let Some(mut buf) = translated_byte_buffer_safe(token, _id as *const u8, 1, true) {
                buf[0][0] = _data as u8;
                return 0;
            }
            -1
        }
        2 => get_current_syscall_count(_id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");

    // 检查 _start 是否按页对齐，且 port 是否合法
    if _start % PAGE_SIZE != 0 || !(1..=0x7).contains(&_port) {
        return -1;
    }

    // _port 权限转换为内存权限
    let mut perm = MapPermission::U;
    if _port & 0b1 != 0 {
        perm |= MapPermission::R;
    }
    if _port & 0b10 != 0 {
        perm |= MapPermission::W;
    }
    if _port & 0b100 != 0 {
        perm |= MapPermission::X;
    }

    if !mmap_current(_start, _len, perm) {
        return -1;
    }

    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    // _start 必须按页对齐
    if _start % PAGE_SIZE != 0 {
        return -1;
    }

    if !munmap_current(_start, _len) {
        return -1;
    }
    0
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

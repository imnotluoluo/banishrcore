//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk,exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,get_syscall_num,get_first_calltime,
        get_current_task_page_table,create_new_map_area,unmap_consecutive_area,
    },
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
use crate::timer::get_time_us;
use crate::task::current_user_token;
use core::mem::size_of;
use crate::mm::translated_byte_buffer;
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    // trace!("kernel: sys_get_time");
    let us = get_time_us();
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, size_of::<TimeVal>());
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let mut time_val_ptr = &time_val as *const _ as *const u8;
    for buffer in buffers {
        unsafe {
            time_val_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            time_val_ptr = time_val_ptr.add(buffer.len());
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {


    let task_info = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: get_syscall_num(),
        time: get_time_us()/1000-get_first_calltime()/1000,
    };//ch3

    let buffers = translated_byte_buffer(
        current_user_token(),
        _ti as *const u8, core::mem::size_of::<TaskInfo>()
    );
    
    let mut task_info_ptr = &task_info as *const _ as *const u8;
    for buffer in buffers {
        unsafe {
            task_info_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            task_info_ptr = task_info_ptr.add(buffer.len());
        }
    }
    
    0
}

// YOUR JOB: Implement mmap.
use crate::config::PAGE_SIZE;
use crate::mm::is_mem_sufficient;
use crate::mm::{VPNRange, VirtAddr, VirtPageNum, MapPermission};
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    // trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    // -1
    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 ||_port & 0x7 ==0{
        return -1;
    }
    if !is_mem_sufficient(_len) {
        return -1;
    }
    let start_va: VirtPageNum = VirtAddr::from(_start).floor();
    let end_va: VirtPageNum = VirtAddr::from(_start + _len).ceil();
    let vpns = VPNRange::new(start_va, end_va);
    for vpn in vpns {
       if let Some(pte) = get_current_task_page_table(vpn) {
            if pte.is_valid() {
                return -1;
            }
       }
    }
    create_new_map_area(
        start_va.into(),
        end_va.into(),
        MapPermission::from_bits_truncate((_port << 1) as u8) | MapPermission::U
    );
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    // trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    // -1
    let start_va: VirtPageNum = VirtAddr::from(_start).floor();
    let end_va: VirtPageNum = VirtAddr::from(_start + _len).ceil();
    let vpns = VPNRange::new(start_va, end_va);
    unmap_consecutive_area(vpns)
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

//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::mm::{frame_alloc, MapPermission, VirtAddr, translated_ptr, translated_mut_ptr};
use crate::task::{current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_tcb_ref_mut, get_current_task_info};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let ts = translated_mut_ptr(current_user_token(), ts);
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    if start & (PAGE_SIZE-1) != 0 {
        // 没有按照页对齐
        return -1;
    }

    if port & (!0x07) != 0 || (port & 0x07) == 0 { return -1; } 

    let start_va: VirtAddr = VirtAddr::from(start).floor().into();
    let end_va: VirtAddr = VirtAddr::from(start + len).ceil().into();

    let mut permission = MapPermission::U;
    if port & 0x01 != 0 { permission.set(MapPermission::R, true); }
    if port & 0x02 != 0 { permission.set(MapPermission::W, true); }
    if port & 0x04 != 0 { permission.set(MapPermission::X, true); }


    if !get_tcb_ref_mut(|tcb| {
        tcb.memory_set.mmap(start_va.into(), end_va.into(), permission)
    })  {
        return -1;
    }
    
    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if !get_tcb_ref_mut(|tcb| {
        tcb.memory_set.munmap(_start.into(), _len.into())
    })  {
        return -1;
    }
    0
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let token = current_user_token();
    let ti = translated_mut_ptr(token, ti);
    match unsafe {ti.as_mut()} {
        Some(ti_mut_ref) => {
            get_current_task_info(ti_mut_ref);
            0
        },
        None => -1
    }
}

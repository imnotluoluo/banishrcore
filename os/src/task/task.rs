//! Types related to task management

use super::TaskContext;


use crate::config::MAX_SYSCALL_NUM;//newnew 对这个crate有点疑惑
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,

    //newnew
    /// The num of syscall times
    pub syscall_num:[u32;MAX_SYSCALL_NUM],
    /// first syscall time
    pub first_calltime:usize,
    /// have be called
    pub have_becalled:u32,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.


use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use crate::config::BIG_STRIDE;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        // insert the new task into a proper position
        let inner = task.inner_exclusive_access();
        let pass = inner.pass;
        // let prio = inner.priority;
        // drop the ownership of inner
        drop(inner);

        let len = self.ready_queue.len();
        for idx in 0..len {
            let queue_task = self.ready_queue.get_mut(idx).unwrap();
            let pass1 = queue_task.inner_exclusive_access().pass;
            // keep the queue head owns the smallest pass
            if pass < pass1 {
                // println!("new task priority: {}, pass: {}, inserted before idx {}", prio, pass, idx);
                self.ready_queue.insert(idx, task);
                return
            }
        }
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}

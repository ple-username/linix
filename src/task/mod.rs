// Task Management Module

use spin::Mutex;
use core::mem::MaybeUninit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[repr(C)]
pub struct TaskContext {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    rbp: u64,
}

pub struct Task {
    pub id: u32,
    pub state: TaskState,
    pub context: TaskContext,
    pub priority: u8,
}

impl Task {
    pub fn new(id: u32, entry_point: u64) -> Self {
        Task {
            id,
            state: TaskState::Ready,
            context: TaskContext {
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
                rsi: 0,
                rdi: 0,
                rsp: 0xFFFF800000010000 + (id as u64) * 0x1000,
                rbp: 0,
            },
            priority: 0,
        }
    }
}

const MAX_TASKS: usize = 32;

pub struct Scheduler {
    tasks: [Option<Task>; MAX_TASKS],
    current_task: usize,
    task_count: u32,
}

impl Scheduler {
    pub const fn new() -> Self {
        const NONE: Option<Task> = None;
        Scheduler {
            tasks: [NONE; MAX_TASKS],
            current_task: 0,
            task_count: 0,
        }
    }

    pub fn create_task(&mut self, entry_point: u64) -> u32 {
        let id = self.task_count;
        self.task_count += 1;

        for i in 0..MAX_TASKS {
            if self.tasks[i].is_none() {
                self.tasks[i] = Some(Task::new(id, entry_point));
                return id;
            }
        }
        0
    }

    pub fn get_current_task(&self) -> Option<&Task> {
        self.tasks[self.current_task].as_ref()
    }

    pub fn switch_task(&mut self) {
        self.current_task = (self.current_task + 1) % MAX_TASKS;
        while self.tasks[self.current_task].is_none() {
            self.current_task = (self.current_task + 1) % MAX_TASKS;
        }
    }
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn init() {
    println!("Task management initialized");
}

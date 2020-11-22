#![allow(dead_code)]
#![allow(unused_variables)]
use super::interrupt::*;
use oorandom;

/// task的最大数量
const NR_TASKS: usize = 5;

/// 定义task的状态，Lab3中task只需要一种状态
const TASK_RUNNING: u64 = 0;

const THREAD_SIZE: usize = 0x1000;

static TASK_SPACE_START: usize = 0x80010000;

static mut SEED: u128 = 15;

#[repr(C)]
#[derive(Clone, Copy, Default)]
/// 进程状态段数据结构
pub struct thread_struct {
    pub ra: u64,
    pub sp: u64,
    pub regs: [u64; 29],
}

#[derive(Clone, Copy)]
/// 进程数据结构
pub struct TaskStruct {
    /// 进程状态 Lab3中进程初始化时置为TASK_RUNNING
    pub state: u64,
    /// 运行剩余时间
    pub counter: u64,
    /// 运行优先级 1最高 5最低
    pub priority: u64,
    pub blocked: bool,
    pub epc: u64,
    /// 进程标识符
    pub pid: usize,
    /// 该进程状态段
    pub thread: thread_struct,
}

pub struct SchedTest {
    tasks: [TaskStruct; NR_TASKS],
    current_task_id: usize,
}

pub static mut SCHED: SchedTest = SchedTest {
    tasks: [TaskStruct {
        state: TASK_RUNNING,
        counter: 0,
        priority: 5,
        blocked: false,
        pid: 0,
        epc: 0,
        thread: thread_struct {
            ra: 0,
            sp: 0,
            regs: [0; 29],
        },
    }; NR_TASKS],
    current_task_id: 0,
};

fn get_randi() -> u64 {
    unsafe {
        SEED += 1;
        let mut rng = oorandom::Rand64::new(SEED);
        rng.rand_u64()
    }
}

impl SchedTest {
    fn init_shced() {
        for id in 1..NR_TASKS {
            unsafe {
                let mut task = &mut SCHED.tasks[id];
                task.pid = id;
                task.thread.sp = (TASK_SPACE_START + THREAD_SIZE * (id + 1)) as u64;
            }
        }
    }

    pub fn task_init() {
        println!("task init...");
        #[cfg(feature = "short_job_first")]
        SchedTest::short_job_first_init();
        #[cfg(feature = "priority")]
        SchedTest::priority_init();
    }

    fn short_job_first_init() {
        SchedTest::init_shced();
        unsafe {
            for id in 1..NR_TASKS {
                let mut task = &mut SCHED.tasks[id];
                task.counter = get_randi() % 5 + 1;
                task.priority = 5;
                task.epc = dead_loop as u64;
                println!(
                    "[PID = {}] Process Create Successfully! counter = {}",
                    task.pid, task.counter
                );
            }
        }
    }

    fn priority_init() {
        SchedTest::init_shced();
        unsafe {
            for id in 1..NR_TASKS {
                let mut task = &mut SCHED.tasks[id];
                task.counter = 8 - id as u64;
                task.priority = 5;
                task.epc = dead_loop as u64;
                println!(
                    "[PID = {}] Process Create Successfully! counter = {} priority = {}",
                    task.pid, task.counter, task.priority
                );
            }
        }
    }

    pub fn do_timer(&mut self, current: &mut Context) {
        #[cfg(feature = "short_job_first")]
        self.short_job_first_do_timer(current);
        #[cfg(feature = "priority")]
        self.priority_do_timer(current);
    }

    fn priority_do_timer(&mut self, current: &mut Context) {
        if self.tasks[self.current_task_id].counter >= 1 {
            self.tasks[self.current_task_id].counter -= 1;
        }
        self.priority_schedule(current);
    }

    fn priority_schedule(&mut self, current: &mut Context) {
        let mut min_id = 4;
        for id in (1..NR_TASKS).rev() {
            let task = &self.tasks[id];
            if task.counter == 0 {
                continue;
            }
            if task.priority < self.tasks[min_id].priority
                || (task.priority == self.tasks[min_id].priority
                    && task.counter < self.tasks[min_id].counter)
            {
                min_id = id;
            }
        }
        if self.tasks[self.current_task_id].counter == 0 && self.current_task_id != 0 {
            self.tasks[self.current_task_id].counter = 8 - self.current_task_id as u64;
            println!(
                "[PID = {}] Reset counter = {}",
                self.current_task_id, self.tasks[self.current_task_id].counter
            );
        }
        self.switch_to(current, min_id);
        for id in 1..NR_TASKS {
            self.tasks[id].priority = get_randi() % 5 + 1;
            println!(
                "[PID = {}] counter = {} priority = {}",
                id, self.tasks[id].counter, self.tasks[id].priority
            );
        }
    }

    fn short_job_first_do_timer(&mut self, current: &mut Context) {
        println!(
            "[PID = {}] Context Calculation: counter = {}",
            self.current_task_id, self.tasks[self.current_task_id].counter
        );
        if self.tasks[self.current_task_id].counter >= 1 {
            self.tasks[self.current_task_id].counter -= 1;
        }
        if self.tasks[self.current_task_id].counter == 0 {
            self.short_job_first_schedule(current);
        }
    }

    fn short_job_first_schedule(&mut self, current: &mut Context) {
        let mut min_id = 0;
        let mut min_counter = core::u64::MAX;
        for id in (1..NR_TASKS).rev() {
            let task = &self.tasks[id];
            if task.counter != 0 && task.counter < min_counter {
                min_id = id;
                min_counter = task.counter;
            }
        }
        if min_id != 0 {
            self.switch_to(current, min_id);
        } else {
            for id in 1..NR_TASKS {
                let mut task = &mut self.tasks[id];
                task.counter = get_randi() % 5 + 1;
                println!("[PID = {}] Reset counter = {}", task.pid, task.counter);
            }
            self.short_job_first_schedule(current);
        }
    }

    fn switch_to(&mut self, current: &mut Context, next_id: usize) {
        if self.current_task_id == next_id {
            return;
        }
        println!(
            "[!] Switch from task {} to task {}, prio: {}, counter: {}",
            self.current_task_id,
            next_id,
            self.tasks[next_id].priority,
            self.tasks[next_id].counter
        );
        let mut current_task = &mut self.tasks[self.current_task_id];
        current_task.thread = current.regs.clone();
        current_task.epc = current.epc;
        current.regs.ra = self.tasks[next_id].thread.ra;
        current.regs.sp = self.tasks[next_id].thread.sp;
        current.regs.regs = self.tasks[next_id].thread.regs;
        current.epc = self.tasks[next_id].epc;
        self.current_task_id = next_id;
    }
}

fn dead_loop() {
    loop {}
}

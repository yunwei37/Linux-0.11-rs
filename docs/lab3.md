# Lab 2 RISC-V64简单的进程调度实现

## 文件结构

lab 文件夹目录结构：

```
├── Cargo.lock
├── Cargo.toml
├── Makefile
└── src
    ├── arch
    │   └── riscv
    │       └── kernel
    │           ├── entry.S
    │           ├── head.S
    │           └── vmlinux.lds
    ├── init
    │   ├── interrupt.rs
    │   ├── mod.rs
    │   └── sched.rs
    ├── lib
    │   ├── console.rs
    │   ├── mod.rs
    │   └── register.rs
    └── main.rs

```

其中新添加了 `sched.rs`，主要处理进程调度相关的内容；

## sched.c进程调度功能实现：

### 数据结构定义

sched.rs
```rs
/// task的最大数量
const NR_TASKS: usize = 5;

/// 定义task的状态，Lab3中task只需要一种状态
const TASK_RUNNING: u64 = 0;

const THREAD_SIZE: usize = 0x1000;

static TASK_SPACE_START: usize = 0x80010000;

static mut SEED: u128 = 13;

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
```

以及用于保存全局信息的数据结构：

sched.rs
```rs
pub struct SchedTest {
    tasks: [TaskStruct; NR_TASKS],
    current_task_id: usize,
}
```

### 在中断处理中添加保存epc的指令

head.S
```s
        csrr t0, sstatus
        csrr t1, sepc
        sd t0,  248(sp)
        sd t1,  256(sp)

        mv      a0, sp

        call supervisor_trap_handler

        ld t0,  248(sp)
        ld t1,  256(sp)
        csrw sstatus, t0
        csrw sepc, t1
```

### 实现 task_init()

这里分别实现了两种 task_init()，使用条件编译指令确定具体调用的 task_init 函数：

公共部分：

sched.rs
```rs
impl SchedTest {
    pub fn task_init() {
        println!("task init...");
        #[cfg(feature = "short_job_first")]
        SchedTest::short_job_first_init();
        #[cfg(feature = "priority")]
        SchedTest::priority_init();
    }

    fn init_shced() {
        for id in 1..NR_TASKS {
            unsafe {
                let mut task = &mut SCHED.tasks[id];
                task.pid = id;
                task.thread.sp = (TASK_SPACE_START + THREAD_SIZE * (id + 1)) as u64;
            }
        }
    }

```

短作业优先非抢占式算法：

sched.rs
```rs

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

```

优先级抢占式算法：

sched.rs
```rs
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
```

###  do_timer()

接口：

sched.rs
```rs
    pub fn do_timer(&mut self, current: &mut Context) {
        #[cfg(feature = "short_job_first")]
        self.short_job_first_do_timer(current);
        #[cfg(feature = "priority")]
        self.priority_do_timer(current);
    }
```

短作业优先非抢占式算法：

sched.rs
```rs
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

```

优先级抢占式算法：

sched.rs
```rs
    fn priority_do_timer(&mut self, current: &mut Context) {
        if self.tasks[self.current_task_id].counter >= 1 {
            self.tasks[self.current_task_id].counter -= 1;
        }
        self.priority_schedule(current);
    }
```

###  schedule()

短作业优先非抢占式算法：

sched.rs
```rs
    fn short_job_first_schedule(&mut self, current: &mut Context) {
        let mut min_id = 0;
        let mut min_counter = core::u64::MAX;
        for id in (1..NR_TASKS).rev() {
            if self.tasks[id].counter != 0 && self.tasks[id].counter < min_counter {
                min_id = id;
                min_counter = self.tasks[id].counter;
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

```

优先级抢占式算法：

sched.rs
```rs
    fn priority_schedule(&mut self, current: &mut Context) {
        let mut min_id = 4;
        for id in (1..NR_TASKS).rev() {
            if self.tasks[id].counter == 0 {
                continue;
            }
            if self.tasks[id].priority < self.tasks[min_id].priority
                || (self.tasks[id].priority == self.tasks[min_id].priority
                    && self.tasks[id].counter < self.tasks[min_id].counter)
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
```

### switch_to

sched.rs
```rs
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
        self.tasks[self.current_task_id].thread = current.regs.clone();
        self.tasks[self.current_task_id].epc = current.epc;
        current.regs.ra = self.tasks[next_id].thread.ra;
        current.regs.sp = self.tasks[next_id].thread.sp;
        current.regs.regs = self.tasks[next_id].thread.regs;
        current.epc = self.tasks[next_id].epc;
        self.current_task_id = next_id;
    }
```

## 时钟中断处理

在 init\interrupt.rs 中：
```rs
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub regs: thread_struct,
    pub status: u64,
    pub epc: u64,
}

#[no_mangle]
pub extern "C" fn supervisor_trap_handler(context: &mut Context) {
    let scause = r_scause();
    // println!("supervisor_trap_handler {} epc {:x}", scause, context.epc);
    const S_TIMER_INTERRUPT: u64 = (1 << 63) | 5;
    if scause == S_TIMER_INTERRUPT {
        unsafe {
            TICKS += 1;
            if TICKS >= 20 {
                shut_down();
            }
            SCHED.do_timer(context);
            w_sie(r_sie() & !SIE_STIE);
            llvm_asm!("ecall");
        }
    } else {
        panic!("unknown supervisor trap: scause {}", scause);
    }
}
```

## makefile

可以使用 `SCHED		:= short_job_first` 这个变量控制具体哪种调度算法，另外一个可选项是 `priority`

另外，由于使用的随机数算法可能有差异，部分结果可能不完全相同。

## 结果

### 短作业优先非抢占式算法

![SJF](SJF.PNG)

### 优先级抢占式算法

![PRIORITY](PRIORITY.PNG)

## 心得体会

发现我好像某次上传失败了但没注意到...又有另外一位交到这里来了...所以迟交了不少...

现在代码实际上很乱，又暂时没有做堆内存分配器导致需要使用一堆全局变量和 unsafe，之后需要修改好一下。

//!
//!  ______    _          _    ____      
//! |  ____|  | |        (_)  / __ \     
//! | |__ __ _| |__  _ __ _  | |  | |___
//! |  __/ _` | '_ \| '__| | | |  | / __|
//! | | | (_| | |_) | |  | | | |__| \__ \
//! |_|  \__,_|_.__/|_|  |_|  \____/|___/
//!                                                                        
//! Copyright (c) 2025, Flo1991
//!
//! BSD 3-Clause License - see LICENSE file for details
//!
//! Author : Florian Wank
//! Creation Date : 24.03.2025
//! File : sched.rs
//!
//! # Short description
//! Scheduler module implementation
//!
//! # Detailed description
//! This module implements the scheduling mechanism.
//! @todo : expand comment!
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use crate::appl::ledm;
use crate::common::util::create_volatile;
use crate::mcal::gpt::Timer6_7;
use core::arch::asm;
use core::num::Wrapping;

//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------
type TaskFunction = fn();
type ProcessFunction = fn();

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------
static mut S_SCHED: SchedData = SchedData {
    active_task_func: u_task0,
    scheduler_timestamp: Wrapping(0),
    scheduler_extended_timestamp: 0,
    urgent_task: u_task0,
    task_idx: 0,
    task_list: [
        f_task0, m_task0, f_task1, l_task0, f_task0, m_task1, f_task1, l_task1, f_task0, m_task2,
        f_task1, l_task2, f_task0, m_task3, f_task1, l_task3, f_task0, m_task4, f_task1, l_task4,
        f_task0, m_task0, f_task1, l_task5, f_task0, m_task1, f_task1, l_task6, f_task0, m_task2,
        f_task1, l_task7, f_task0, m_task3, f_task1, l_task8, f_task0, m_task4, f_task1, l_task9,
    ],
    tasks_max_runtime: [u32::MIN; TASK_NUM],
    tasks_min_runtime: [u32::MAX; TASK_NUM],
    tasks_period_us: [0; TASK_NUM],

    stack_pid00: StackTypePid00 {
        stack: [0; PROC_PID00_STACK_SIZE],
    },
    stack_pid01: StackTypePid01 {
        stack: [0; PROC_PID01_STACK_SIZE],
    },
    stack_pid02: StackTypePid02 {
        stack: [0; PROC_PID02_STACK_SIZE],
    },

    process_table: [
        Process {
            pid: PidT::Pid00,
            stack_ptr: 0,
            process_func: proc_pid00,
        },
        Process {
            pid: PidT::Pid01,
            stack_ptr: 0,
            process_func: proc_pid01,
        },
        Process {
            pid: PidT::Pid02,
            stack_ptr: 0,
            process_func: proc_pid02,
        },
    ],
    main_process: Process {
        pid: PidT::PidMain,
        stack_ptr: 0,
        process_func: proc_pid00,
    },
    active_process: PidT::PidMain,
};

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------
///number of element per context on stack (registers and core status content)
const NUM_OF_STACK_ELEMS: usize = 15;

///stack size of stack pid00
const PROC_PID00_STACK_SIZE: usize = 128;
///stack size of stack pid01
const PROC_PID01_STACK_SIZE: usize = 128;
///stack size of stack pid02
const PROC_PID02_STACK_SIZE: usize = 128;

///number of task slots
const TASK_NUM_OF_SLOTS: usize = 40;
///number of different tasks
const TASK_NUM: usize = 17;

///timeslot duration for each timeslot; each timeslot starts with the urgent task followed by one task
///the time is in us
const TASK_SCHEDULE_TIMESLOT_TIME_US: u32 = 250;

///reference count value to which the scheduler is synced; this value ensures the timeslot matching;
///CAUTION: the user has to ensure that the mcu specific timer counter can reach this value!
///NOTE : this value compares to the counter value directly; the counter wrap value must be greater (because used to check if
///scheduling timing is in limits!)
const TASK_SCHED_CNT_START_REF_VAL: i32 =
    (crate::mcal::rcc::F_CPU_HZ / (1000000i32)) * (TASK_SCHEDULE_TIMESLOT_TIME_US as i32);

///time for urgent task in us
const TASK_SCHEDULE_URGENT_TASK_TIME_US: u32 = 25;

///time amount that is used for urgent task; if the urgent task is faster, wait this value to be deterministic!
const TASK_SCHED_CNT_URGENT_REF_VAL: i32 =
    (crate::mcal::rcc::F_CPU_HZ / (1000000i32)) * (TASK_SCHEDULE_URGENT_TASK_TIME_US as i32);

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------
#[derive(Copy, Clone)]
enum TaskID {
    Ftask0 = 0,
    Ftask1 = 1,

    Mtask0 = 2,
    Mtask1 = 3,
    Mtask2 = 4,
    Mtask3 = 5,
    Mtask4 = 6,

    Ltask0 = 7,
    Ltask1 = 8,
    Ltask2 = 9,
    Ltask3 = 10,
    Ltask4 = 11,
    Ltask5 = 12,
    Ltask6 = 13,
    Ltask7 = 14,
    Ltask8 = 15,
    Ltask9 = 16,
}

#[derive(Copy, Clone)]
pub enum PidT {
    Pid00 = 0,
    Pid01 = 1,
    Pid02 = 2,
    PidMain = 0xE3,
}

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SchedData {
    active_task_func: TaskFunction,
    scheduler_timestamp: Wrapping<u32>,
    scheduler_extended_timestamp: u64,
    urgent_task: TaskFunction,
    task_idx: u32,
    task_list: [TaskFunction; TASK_NUM_OF_SLOTS],
    tasks_max_runtime: [u32; TASK_NUM],
    tasks_min_runtime: [u32; TASK_NUM],
    tasks_period_us: [u32; TASK_NUM],

    stack_pid00: StackTypePid00,
    stack_pid01: StackTypePid01,
    stack_pid02: StackTypePid02,

    process_table: [Process; 3],
    main_process: Process,
    active_process: PidT,
}

/// definition of a process
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Process {
    pid: PidT,
    stack_ptr: u32,
    process_func: ProcessFunction,
}

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub union StackTypePid00 {
    stack: [u32; PROC_PID00_STACK_SIZE],
}

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub union StackTypePid01 {
    stack: [u32; PROC_PID01_STACK_SIZE],
}

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub union StackTypePid02 {
    stack: [u32; PROC_PID02_STACK_SIZE],
}

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------
macro_rules! process_wait_time_elapse_us {
    ($WAIT_TIME_US:expr) => {
        // here the internal macro begins; create a new scope to avoid duplication issues
        {
            let ref_time : Wrapping<u32> = get_timestamp_us();
            let time_to_elapse: Wrapping<u32> = Wrapping($WAIT_TIME_US);

            while !is_elapsed_us(ref_time, time_to_elapse) {
                sched_yield();
            }
            
        }
  
    };
}

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------
#[allow(clippy::fn_to_numeric_cast)]
pub fn s_init() {
    unsafe {
        S_SCHED.tasks_period_us[TaskID::Ftask0 as usize] = 500;
        S_SCHED.tasks_period_us[TaskID::Ftask1 as usize] = 500;
        S_SCHED.tasks_period_us[TaskID::Mtask0 as usize] = 5000;
        S_SCHED.tasks_period_us[TaskID::Mtask1 as usize] = 5000;
        S_SCHED.tasks_period_us[TaskID::Mtask2 as usize] = 5000;
        S_SCHED.tasks_period_us[TaskID::Mtask3 as usize] = 5000;
        S_SCHED.tasks_period_us[TaskID::Mtask4 as usize] = 5000;
        S_SCHED.tasks_period_us[TaskID::Ltask0 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask1 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask2 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask3 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask4 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask5 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask6 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask7 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask8 as usize] = 10000;
        S_SCHED.tasks_period_us[TaskID::Ltask9 as usize] = 10000;

        S_SCHED.process_table[0].stack_ptr = (&mut S_SCHED.stack_pid00.stack
            [PROC_PID00_STACK_SIZE - NUM_OF_STACK_ELEMS]
            as *mut u32) as u32;
        S_SCHED.process_table[1].stack_ptr = (&mut S_SCHED.stack_pid01.stack
            [PROC_PID01_STACK_SIZE - NUM_OF_STACK_ELEMS]
            as *mut u32) as u32;
        S_SCHED.process_table[2].stack_ptr = (&mut S_SCHED.stack_pid02.stack
            [PROC_PID02_STACK_SIZE - NUM_OF_STACK_ELEMS]
            as *mut u32) as u32;

        S_SCHED.stack_pid00.stack[0] = 0xdeadbeef;
        S_SCHED.stack_pid00.stack[1] = 0xabad1dea;
        S_SCHED.stack_pid00.stack[PROC_PID00_STACK_SIZE - NUM_OF_STACK_ELEMS + 1] =
            (S_SCHED.process_table[0].process_func) as u32;

        S_SCHED.stack_pid01.stack[0] = 0xdeadbeef;
        S_SCHED.stack_pid01.stack[1] = 0xabad1dea;
        S_SCHED.stack_pid01.stack[PROC_PID01_STACK_SIZE - NUM_OF_STACK_ELEMS + 1] =
            (S_SCHED.process_table[1].process_func) as u32;

        S_SCHED.stack_pid02.stack[0] = 0xdeadbeef;
        S_SCHED.stack_pid02.stack[1] = 0xabad1dea;
        S_SCHED.stack_pid02.stack[PROC_PID02_STACK_SIZE - NUM_OF_STACK_ELEMS + 1] =
            (S_SCHED.process_table[2].process_func) as u32;
    }
}

pub fn sched_run() {
    unsafe {
        loop {
            S_SCHED.task_idx = 0;
            #[allow(static_mut_refs)]
            while S_SCHED.task_idx < S_SCHED.task_list.len() as u32 {
                if Timer6_7::inst_6().get_cnt_value() >= TASK_SCHED_CNT_START_REF_VAL as u32 {
                    Timer6_7::inst_6().reset_cnt_value();
                    S_SCHED.active_task_func = S_SCHED.urgent_task;
                    (S_SCHED.urgent_task)();
                    while Timer6_7::inst_6().get_cnt_value() < TASK_SCHED_CNT_URGENT_REF_VAL as u32
                    {
                    }
                    S_SCHED.active_task_func = S_SCHED.task_list[S_SCHED.task_idx as usize];
                    (S_SCHED.active_task_func)();

                    if Timer6_7::inst_6().timer_elapsed() {
                        asm!(".rept 3  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
                    }
                    S_SCHED.task_idx += 1;
                }
            }
        }
    }
}

#[allow(dead_code)]
fn get_timestamp_us() -> Wrapping<u32> {
    unsafe {
        S_SCHED.scheduler_timestamp
    }
}

#[allow(dead_code)]
fn get_elapsed_us(timestamp: Wrapping<u32>) -> Wrapping<u32> {
    unsafe {
        S_SCHED.scheduler_timestamp - timestamp
    }
}

#[allow(dead_code)]
fn is_elapsed_us(
    timestamp: Wrapping<u32>,
    time_to_elapse: Wrapping<u32>,
) -> bool {
    unsafe {S_SCHED.scheduler_timestamp - timestamp > time_to_elapse}
}

fn get_task_sched_times(task_id: TaskID) {
    let task_runtime: u32 = Timer6_7::inst_6().get_cnt_value() - 200;
    unsafe {
        S_SCHED.tasks_max_runtime[task_id as usize] =
            if S_SCHED.tasks_max_runtime[task_id as usize] > task_runtime {
                S_SCHED.tasks_max_runtime[task_id as usize]
            } else {
                task_runtime
            };
        S_SCHED.tasks_min_runtime[task_id as usize] =
            if S_SCHED.tasks_min_runtime[task_id as usize] < task_runtime {
                S_SCHED.tasks_min_runtime[task_id as usize]
            } else {
                task_runtime
            };
    }
}

#[inline(never)]
fn change_context_yield(active_process_stack_addr: *mut u32, next_process_stack_addr: u32) {
    unsafe {
        change_context_internal(active_process_stack_addr, next_process_stack_addr);
    }
}

#[inline(never)]
fn change_context_process(active_process_stack_addr: *mut u32, next_process_stack_addr: u32) {
    unsafe {
        change_context_internal(active_process_stack_addr, next_process_stack_addr);
    }
}

#[unsafe(naked)]
#[allow(unused_variables)]
pub unsafe extern "C" fn change_context_internal(
    active_process_stack_addr: *mut u32,
    next_process_stack_addr: u32,
) {
    unsafe {
        core::arch::naked_asm!(
                /* Push all registers (not r13, because r13 is stackpointer and is saved in variable) */
                "PUSH    {{r0}}\n",
                "PUSH    {{r1}}\n",
                "PUSH    {{r2}}\n",
                "PUSH    {{r3}}\n",
                "PUSH    {{r4}}\n",
                "PUSH    {{r5}}\n",
                "PUSH    {{r6}}\n",
                "PUSH    {{r7}}\n",
                /* save r8 */
                "MOV     r4,r8\n",
                "PUSH    {{r4}}\n",
                /* save r9 */
                "MOV     r4,r9\n",
                "PUSH    {{r4}}\n",
                /* save r10 */
                "MOV     r4,r10\n",
                "PUSH    {{r4}}\n",
                /* save r11 */
                "MOV     r4,r11\n",
                "PUSH    {{r4}}\n",
                /* save r12 */
                "MOV     r4,r12\n",
                "PUSH    {{r4}}\n",
                /* save r14 */
                "MOV     r4,r14\n",
                "PUSH    {{r4}}\n",
                /* save mcu status */
                "MRS r4, APSR \n",
                "PUSH    {{r4}}\n",

                //"BL      changeContext_body\n",
                "mrs r2, msp",
                "str r2, [r0]",
                "msr msp, r1",
                "isb 0xF",

                /* restore mcu status */
                "POP {{r4}}   \n",
                "MSR APSR_nzcvq, r4 \n", // use APSR_nzcvqg is dsp instructions are supported by mcu (--> GE bits)
                /* restore r14 */
                "POP {{r4}}   \n",
                "MOV r14, r4\n",
                /* restore r12 */
                "POP {{r4}}   \n",
                "MOV r12, r4\n",
                /* restore r11 */
                "POP {{r4}}   \n",
                "MOV r11, r4\n",
                /* restore r10 */
                "POP {{r4}}   \n",
                "MOV r10, r4\n",
                /* restore r19 */
                "POP {{r4}}   \n",
                "MOV r9, r4\n",
                /* restore r8 */
                "POP {{r4}}   \n",
                "MOV r8, r4\n",
                /* restore r8 */
                "POP {{r7}}   \n",
                "POP {{r6}}   \n",
                "POP {{r5}}   \n",
                "POP {{r4}}   \n",
                "POP {{r3}}   \n",
                "POP {{r2}}   \n",
                "POP {{r1}}   \n",
                "POP {{r0}}   \n",

                /* jump to matching lr */
                "BX      r14"

        );
    }
}

#[inline(always)]
fn sched_get_stack_ptr(pid: PidT) -> *mut u32 {
    match pid {
        PidT::Pid00 => unsafe { &mut S_SCHED.process_table[0].stack_ptr as *mut u32 },
        PidT::Pid01 => unsafe { &mut S_SCHED.process_table[1].stack_ptr as *mut u32 },
        PidT::Pid02 => unsafe { &mut S_SCHED.process_table[2].stack_ptr as *mut u32 },
        PidT::PidMain => unsafe { &raw mut S_SCHED.main_process.stack_ptr },
    }
}

#[inline(always)]
fn sched_get_stack_ptr_val(pid: PidT) -> u32 {
    match pid {
        PidT::Pid00 => unsafe { S_SCHED.process_table[0].stack_ptr },
        PidT::Pid01 => unsafe { S_SCHED.process_table[1].stack_ptr },
        PidT::Pid02 => unsafe { S_SCHED.process_table[2].stack_ptr },
        PidT::PidMain => unsafe { S_SCHED.main_process.stack_ptr },
    }
}

#[inline(never)]
pub fn sched_yield() {
    unsafe {
        let active_process_stack_addr: *mut u32 = sched_get_stack_ptr(S_SCHED.active_process);
        S_SCHED.active_process = PidT::PidMain;
        change_context_yield(active_process_stack_addr, S_SCHED.main_process.stack_ptr);
    };
}

#[inline(never)]
pub fn run_process(process_id: PidT) {
    unsafe {
        let next_process_stack_addr: u32 = sched_get_stack_ptr_val(process_id);
        S_SCHED.active_process = process_id;
        #[allow(static_mut_refs)]
        change_context_process(
            &mut S_SCHED.main_process.stack_ptr as *mut u32,
            next_process_stack_addr,
        );
    };
}

fn u_task0() {
    unsafe {
        if S_SCHED.scheduler_timestamp + Wrapping(TASK_SCHEDULE_TIMESLOT_TIME_US)
            < S_SCHED.scheduler_timestamp
        {
            S_SCHED.scheduler_extended_timestamp = S_SCHED.scheduler_extended_timestamp
                + (S_SCHED.scheduler_timestamp).0 as u64
                + TASK_SCHEDULE_TIMESLOT_TIME_US as u64;
        }
        S_SCHED.scheduler_timestamp += Wrapping(TASK_SCHEDULE_TIMESLOT_TIME_US);
    }
}

fn f_task0() {
    run_process(PidT::Pid00);
    get_task_sched_times(TaskID::Ftask0);
}

fn f_task1() {
    get_task_sched_times(TaskID::Ftask1);
}

fn m_task0() {
    run_process(PidT::Pid02);
    get_task_sched_times(TaskID::Mtask0);
}

fn m_task1() {
    get_task_sched_times(TaskID::Mtask1);
}

fn m_task2() {
    get_task_sched_times(TaskID::Mtask2);
}

fn m_task3() {
    get_task_sched_times(TaskID::Mtask3);
}

fn m_task4() {
    get_task_sched_times(TaskID::Mtask4);
}

fn l_task0() {
    get_task_sched_times(TaskID::Ltask0);
}

fn l_task1() {
    run_process(PidT::Pid01);
    get_task_sched_times(TaskID::Ltask1);
}

fn l_task2() {
    get_task_sched_times(TaskID::Ltask2);
}

fn l_task3() {
    get_task_sched_times(TaskID::Ltask3);
}

fn l_task4() {
    get_task_sched_times(TaskID::Ltask4);
}

fn l_task5() {
    get_task_sched_times(TaskID::Ltask5);
}

fn l_task6() {
    get_task_sched_times(TaskID::Ltask6);
}

fn l_task7() {
    get_task_sched_times(TaskID::Ltask7);
}

fn l_task8() {
    get_task_sched_times(TaskID::Ltask8);
}

fn l_task9() {
    get_task_sched_times(TaskID::Ltask9);
}

#[inline(never)]
fn proc_pid00() {
    create_volatile!(u32, val, 5);

    loop {
        //@todo : prevent code reordering!
        val += 1;
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }

        sched_yield();

        

        if val > 8 {
            val = 0;
        }
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }

        sched_yield();

        val = val.saturating_add(1);
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }
    }
}

#[inline(never)]
fn proc_pid01() {
    create_volatile!(u32, val, 5);

    loop {
        //@todo : prevent code reordering!
        val += 2;
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }

        process_wait_time_elapse_us!(1000);
        ledm::ledm_task();
        process_wait_time_elapse_us!(1000);

        if val > 8 {
            val = 0;
        }
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }

        sched_yield();

        val = val.saturating_add(2);
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }
    }
}

#[inline(never)]
fn proc_pid02() {
    create_volatile!(u32, val, 5);

    loop {
        //@todo : prevent code reordering!
        val += 5;
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }

        sched_yield();

        if val > 8 {
            val = 0;
        }
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }

        sched_yield();

        val = val.saturating_add(5);
        unsafe {
            asm!(".rept 5  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }
    }
}

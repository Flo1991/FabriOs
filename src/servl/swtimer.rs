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
//! File : swtimer.rs
//!
//! # Short description
//! Software timer implementation
//!
//! # Detailed description
//! For time triggered tasks that need to be executed slow in comparison to the scheduling, this module
//! provides software timers. The software timer implementation is just counter based - so the timing
//! actually depends on the software timer configuration and the task timing in which the software
//! timer handling function is called.
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use crate::appl::ledm;
use crate::rte;
use crate::rte::RteData;

//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------
type TimerFunction = fn();

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Copy, Clone)]
pub enum States {
    Stopped,
    Run,
}

pub enum TimerId {
    LedmBlinkTimer = 0,
}

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
#[derive(Copy, Clone)]
pub struct ToutTimer {
    pub current_val: u32,
    pub end_val: u32,
    pub state: States,
    pub callback: TimerFunction,
}

impl ToutTimer {
    #[allow(dead_code)]
    pub fn reset(timer: TimerId, rt_data: &mut RteData) {
        rt_data.swtimer_data.timers[timer as usize].current_val = 0;
    }
    #[allow(dead_code)]
    pub fn start(timer: TimerId, rt_data: &mut RteData) {
        rt_data.swtimer_data.timers[timer as usize].state = States::Run;
    }
    #[allow(dead_code)]
    pub fn stop(timer: TimerId, rt_data: &mut RteData) {
        rt_data.swtimer_data.timers[timer as usize].state = States::Stopped;
    }
    #[allow(dead_code)]
    pub fn set_end_value(timer: TimerId, rt_data: &mut RteData, endval: u32) {
        let idx: usize = timer as usize;
        rt_data.swtimer_data.timers[idx].state = States::Stopped;
        rt_data.swtimer_data.timers[idx].current_val = 0;
        rt_data.swtimer_data.timers[idx].end_val = endval;
    }
    #[allow(dead_code)]
    pub fn handle_stop(timer: TimerId, rt_data: &mut RteData) {
        let idx: usize = timer as usize;
        if rt_data.swtimer_data.timers[idx].state == States::Run {
            if rt_data.swtimer_data.timers[idx].current_val
                >= rt_data.swtimer_data.timers[idx].end_val
            {
                (rt_data.swtimer_data.timers[idx].callback)();
                rt_data.swtimer_data.timers[idx].state = States::Stopped;
            } else {
                rt_data.swtimer_data.timers[idx].current_val += 1;
            }
        }
    }
    pub fn handle_repeat(timer: TimerId) -> Result<bool, u32> {
        unsafe {
            let idx: usize = timer as usize;
            if rte::RTE_D.swtimer_data.timers[idx].state == States::Run {
                if rte::RTE_D.swtimer_data.timers[idx].current_val
                    >= rte::RTE_D.swtimer_data.timers[idx].end_val
                {
                    (rte::RTE_D.swtimer_data.timers[idx].callback)();
                    rte::RTE_D.swtimer_data.timers[idx].current_val = 0;
                    Ok(true)
                } else {
                    rte::RTE_D.swtimer_data.timers[idx].current_val += 1;
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    }
}
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
#[derive(Copy, Clone)]
pub struct TimerData {
    timers: [ToutTimer; 1],
}

impl TimerData {
    pub const fn init() -> Self {
        Self {
            timers: [ToutTimer {
                current_val: 0,
                end_val: 50,
                state: States::Run,
                callback: ledm::ledm_blink_timer_callback,
            }],
        }
    }
}
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------

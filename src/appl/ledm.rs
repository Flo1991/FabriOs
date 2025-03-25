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
//! File : ledm.rs
//!
//! # Short description
//! Led manager module
//!
//! # Detailed description
//! Module to handle leds. Leds may be solid or blink
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use crate::common::util::clear_reg_bitmsk;
use crate::common::util::set_reg_bitmsk;
use crate::mcal::gpio::GpioAB;
use crate::rte;
use crate::servl::swtimer::TimerId;
use crate::servl::swtimer::ToutTimer;

//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum LedState {
    Off,
    On,
    BlinkOff,
    BlinkOn, //first param is used for timestamp, second for timestamp handling, third for actual state in blink
}

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Led {
    state: LedState,
    odr_adr: u32,
    pinmask: u32,
}
#[derive(Copy, Clone)]
pub struct LedmData {
    leds: [Led; 1],
}

impl LedmData {
    #[allow(dead_code)]
    pub fn config() -> Self {
        Self {
            leds: [Led {
                state: LedState::BlinkOff,
                odr_adr: GpioAB::inst_a().odr,
                pinmask: 1 << 5,
            }],
        }
    }

    pub const fn init() -> Self {
        Self {
            leds: [Led {
                state: LedState::BlinkOff,
                odr_adr: 0x4800_0000,
                pinmask: 1 << 5,
            }],
        }
    }

    #[allow(dead_code)]
    pub fn get_leds(&self) -> &[Led; 1] {
        &self.leds
    }
    #[allow(dead_code)]
    pub fn set_leds_state(&mut self, index: usize, state: LedState) {
        self.leds[index].state = state;
    }
}

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------

pub fn ledm_task() {
    ToutTimer::handle_repeat(TimerId::LedmBlinkTimer)
}

pub fn ledm_blink_timer_callback() {
    unsafe {
        let mut idx: u32 = 0;
        #[allow(static_mut_refs)]
        while idx < rte::RTE_D.ledm_data.leds.len() as u32 {
            if rte::RTE_D.ledm_data.leds[idx as usize].state == LedState::BlinkOff {
                clear_reg_bitmsk(&mut GpioAB::inst_a().odr, 1 << 5);
                rte::RTE_D.ledm_data.leds[idx as usize].state = LedState::BlinkOn;
            } else if rte::RTE_D.ledm_data.leds[idx as usize].state == LedState::BlinkOn {
                set_reg_bitmsk(&mut GpioAB::inst_a().odr, 1 << 5);
                rte::RTE_D.ledm_data.leds[idx as usize].state = LedState::BlinkOff;
            }
            idx += 1;
        }
    }

    //in case of an error need to inform a failure manager here
}

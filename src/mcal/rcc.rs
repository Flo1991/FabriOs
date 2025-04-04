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
//! File : rcc.rs
//!
//! # Short description
//! Rcc module implementation for the Mcu Rcc peripheral
//!
//! # Detailed description
//! @todo
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------

///Mcu/cpu operation frequency in Hz; 8 MHz -> 8_000_000
pub const F_CPU_HZ: i32 = 8_000_000;

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

/// register structure for Rcc module registers; need C representation for correct memory layout
#[repr(C)]
pub struct Rcc {
    pub cr: u32,
    pub cfgr: u32,
    pub cir: u32,
    pub apb2rstr: u32,
    pub apb1rstr: u32,
    pub ahbenr: u32,
    pub apb2enr: u32,
    pub apb1enr: u32,
    pub bdcr: u32,
    pub csr: u32,
    pub ahbrstr: u32,
    pub cfgr2: u32,
    pub cfgr3: u32,
    pub cr2: u32,
}

impl Rcc {
    /// method to the instance of Rcc
    pub fn inst() -> &'static mut Rcc {
        unsafe { &mut *(0x4002_1000 as *mut Rcc) }
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

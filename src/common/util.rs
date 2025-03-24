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
//! File : util.rs
//!
//! # Short description
//! Utility module containing helper functionality
//!
//! # Detailed description
//! @todo
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use core::ptr;

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

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------

///this macro implementation is used to create something like a volatile variable in processes;
///in fact this does not change the actual data setup, but the rust compiler seems to assume a
///once written volatile variable should always be handled as volatile. This behavior prevents a reordering
///of code in processes. If a variable is only defined locally multiple yields in sequence with modifications on
///the same varialbe may lead to reorder so that the yields are not located as assumed!
macro_rules! create_volatile {
    ($TYPE:ident, $VARNAME:ident, $VALUE:expr) => {
        let mut $VARNAME: $TYPE = $VALUE;
        unsafe { core::ptr::write_volatile(&mut $VARNAME, $VARNAME) };
    };
}

pub(crate) use create_volatile;

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------

/// # Safety
///
/// set the bit mask to the given u32 address which represents a
/// register address; is unsafe due to hardware access
pub fn set_reg_bitmsk(register: &mut u32, bitmsk: u32) {
    unsafe {
        let mut regval = ptr::read_volatile(register);
        regval |= bitmsk;
        ptr::write_volatile(register, regval);
    };
}

/// # Safety
///
/// clear the bit mask to the given u32 address which represents a
/// register address; is unsafe due to hardware access
pub fn clear_reg_bitmsk(register: &mut u32, bitmsk: u32) {
    unsafe {
        let mut regval = ptr::read_volatile(register);
        regval &= !(bitmsk);
        ptr::write_volatile(register, regval);
    };
}

/// # Safety
///
/// modify register bits in a register with the modification mask that
/// has to have 1s set at the bitpositions of the bits that should be
/// modified; the actual value to write to those bits are in derived from reg_val
/// example: if regval is 0b1011 and mod_mask is 0b1100, only bits 2 and 3
/// are updated; bit 3 will get set, bit 2 will get cleared:
/// is unsafe due to hardware access
#[allow(dead_code)]
pub fn modify_reg_bits(register: &mut u32, mod_mask: u32, reg_val: u32) {
    unsafe {
        let mut regval = ptr::read_volatile(register);
        /* only keep the bits to modify */
        let del_msk: u32 = reg_val ^ mod_mask;
        regval &= !(del_msk);
        let set_msk: u32 = reg_val & mod_mask;
        regval |= set_msk;
        ptr::write_volatile(register, regval);
    };
}

/// # Safety
///
/// set the value to the given register; is unsafe due to hardware access
#[allow(dead_code)]
pub fn set_reg(register: &mut u32, reg_val: u32) {
    unsafe {
        ptr::write_volatile(register, reg_val);
    };
}

/// # Safety
///
/// get the value of the given register; is unsafe due to hardware access
#[allow(dead_code)]
pub fn get_reg(register: &mut u32) -> u32 {
    unsafe { ptr::read_volatile(register) }
}

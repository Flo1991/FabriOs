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
//! Creation Date : 21.03.2025
//! File : template.rs
//! 
//! # Short description
//! 
//! # Detailed description
//! 
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

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Errors {
    Unknown,
}

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------

macro_rules! create_rw_regstruct{
    ([$($dtype:ident),+]) => {
        $(pub struct $dtype(u32);
        impl $dtype{
            #[allow(dead_code)]
            pub const fn comb(vals : &[$dtype]) -> $dtype{
                let mut buildval = 0;
                let mut i = 0;
                while i < vals.len()
                {
                    buildval |= vals[i].0;
                    i += 1;
                }
                let retval: $dtype = $dtype(buildval);
                retval
            }

            #[allow(dead_code)]
            pub fn set(&mut self, value: Self) {
                set_reg(&mut self.0, value.0);
            }

            #[allow(dead_code)]
            pub fn set_msk(&mut self, msk: Self) {
                set_reg_bitmsk(&mut self.0, msk.0);
            }

            #[allow(dead_code)]
            pub fn clr_msk(&mut self, msk: Self) {
                clr_reg_bitmsk(&mut self.0, msk.0);
            }

            #[allow(dead_code)]
            pub fn modify(&mut self, mod_mask: Self, update_val: Self) {
                modify_reg_bits(&mut self.0, mod_mask.0, update_val.0);
            }

            #[allow(dead_code)]
            pub fn get(&self) -> Self {
                let mut retval : Self = Self(0);
                retval.0 = get_reg(&self.0);
                retval
            }
        })*
    }
}

macro_rules! create_ro_regstruct{
    ([$($dtype:ident),+]) => {
        $(pub struct $dtype(u32);
        impl $dtype{
            #[allow(dead_code)]
            pub const fn comb(vals : &[$dtype]) -> $dtype{
                let mut buildval = 0;
                let mut i = 0;
                while i < vals.len()
                {
                    buildval |= vals[i].0;
                    i += 1;
                }
                let retval: $dtype = $dtype(buildval);
                retval
            }

            #[allow(dead_code)]
            pub fn get(&self) -> Self {
                let mut retval : Self = Self(0);
                retval.0 = get_reg(&self.0);
                retval
            }
        })*
    }
}

macro_rules! create_wo_regstruct{
    ([$($dtype:ident),+]) => {
        $(pub struct $dtype(u32);
        impl $dtype{
            #[allow(dead_code)]
            pub const fn comb(vals : &[$dtype]) -> $dtype{
                let mut buildval = 0;
                let mut i = 0;
                while i < vals.len()
                {
                    buildval |= vals[i].0;
                    i += 1;
                }
                let retval: $dtype = $dtype(buildval);
                retval
            }

            #[allow(dead_code)]
            pub fn set(&mut self, value: Self) {
                set_reg(&mut self.0, value.0);
            }
        })*
    }
}

pub(crate) use create_rw_regstruct;
pub(crate) use create_ro_regstruct;
pub(crate) use create_wo_regstruct;

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
pub fn clr_reg_bitmsk(register: &mut u32, bitmsk: u32) {
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
pub fn get_reg(register: &u32) -> u32 {
    unsafe { ptr::read_volatile(register) }
}

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
//! File : gpio.rs
//!
//! # Short description
//! Gpio module implementation
//!
//! # Detailed description
//! @todo
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use crate::common::util::{clr_reg_bitmsk, get_reg, set_reg_bitmsk, create_setters ,create_clears, create_getters, create_regstruct};
use crate::mcal::rcc::Rcc;
//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------

pub const _MODER_INPUT: u32 = 0;
pub const _MODER_OUTPUT: u32 = 1;
pub const _MODER_ALTERNATE: u32 = 2;
pub const _MODER_ANALOG: u32 = 3;

pub const _MODER0_INPUT: Moder = Moder(_MODER_INPUT);
pub const _MODER0_OUTPUT: Moder = Moder(_MODER_OUTPUT);
pub const _MODER0_ALTERNATE: Moder = Moder(_MODER_ALTERNATE);
pub const _MODER0_ANALOG: Moder = Moder(_MODER_ANALOG);

pub const _MODER1_INPUT: Moder = Moder(_MODER_INPUT << (1 << 1));
pub const _MODER1_OUTPUT: Moder = Moder(_MODER_OUTPUT << (1 << 1));
pub const _MODER1_ALTERNATE: Moder = Moder(_MODER_ALTERNATE << (1 << 1));
pub const _MODER1_ANALOG: Moder = Moder(_MODER_ANALOG << (1 << 1));

pub const _MODER2_INPUT: Moder = Moder(_MODER_INPUT << (2 << 1));
pub const _MODER2_OUTPUT: Moder = Moder(_MODER_OUTPUT << (2 << 1));
pub const _MODER2_ALTERNATE: Moder = Moder(_MODER_ALTERNATE << (2 << 1));
pub const _MODER2_ANALOG: Moder = Moder(_MODER_ANALOG << (2 << 1));

pub const _MODER3_INPUT: Moder = Moder(_MODER_INPUT << (3 << 1));
pub const _MODER3_OUTPUT: Moder = Moder(_MODER_OUTPUT << (3 << 1));
pub const _MODER3_ALTERNATE: Moder = Moder(_MODER_ALTERNATE << (3 << 1));
pub const _MODER3_ANALOG: Moder = Moder(_MODER_ANALOG << (3 << 1));

pub const _MODER4_INPUT: Moder = Moder(_MODER_INPUT << (4 << 1));
pub const _MODER4_OUTPUT: Moder = Moder(_MODER_OUTPUT << (4 << 1));
pub const _MODER4_ALTERNATE: Moder = Moder(_MODER_ALTERNATE << (4 << 1));
pub const _MODER4_ANALOG: Moder = Moder(_MODER_ANALOG << (4 << 1));

pub const _MODER5_INPUT: Moder = Moder(_MODER_INPUT << (5 << 1));
pub const _MODER5_OUTPUT: Moder = Moder(_MODER_OUTPUT << (5 << 1));
pub const _MODER5_ALTERNATE: Moder = Moder(_MODER_ALTERNATE << (5 << 1));
pub const _MODER5_ANALOG: Moder = Moder(_MODER_ANALOG << (5 << 1));

pub const _ODR_PIN0: Odr = Odr(1);
pub const _ODR_PIN1: Odr = Odr(1 << 1);
pub const _ODR_PIN2: Odr = Odr(1 << 2);
pub const _ODR_PIN3: Odr = Odr(1 << 3);
pub const _ODR_PIN4: Odr = Odr(1 << 4);
pub const _ODR_PIN5: Odr = Odr(1 << 5);

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------
#[allow(dead_code)]
enum Gpio{
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Gpio {
    const fn base (self) -> u32 {
        match self {
            Gpio::A => 0x4800_0000,
            Gpio::B => 0x4800_0400,
            Gpio::C => 0x4800_0800,
            Gpio::D => 0x4800_0C00,
            Gpio::E => 0x4800_1000,
            Gpio::F => 0x4800_1400,
        }
    }
}

#[allow(dead_code)]
pub enum Pin{
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
}

impl Pin {
    const fn obtain_odr (self) -> (Gpio, Odr) {
        match self {
            Pin::A0 => (Gpio::A, _ODR_PIN0),
            Pin::A1 => (Gpio::A, _ODR_PIN1),
            Pin::A2 => (Gpio::A, _ODR_PIN2),
            Pin::A3 => (Gpio::A, _ODR_PIN3),
            Pin::A4 => (Gpio::A, _ODR_PIN4),
            Pin::A5 => (Gpio::A, _ODR_PIN5),
        }
    }
    #[allow(dead_code)]
    pub fn set(self) {
        let io_pair: (Gpio, Odr) = self.obtain_odr();
        match io_pair.0 {
            Gpio::A => {
                GpioAB::inst_a().odr_set(io_pair.1);
            },
            Gpio::B => {},
            Gpio::C => {},
            Gpio::D => {},
            Gpio::E => {},
            Gpio::F => {},
        }
    }
    #[allow(dead_code)]
    pub fn clr(self) {
        let io_pair: (Gpio, Odr) = self.obtain_odr();
        match io_pair.0 {
            Gpio::A => {
                GpioAB::inst_a().odr_clr(io_pair.1);
            },
            Gpio::B => {},
            Gpio::C => {},
            Gpio::D => {},
            Gpio::E => {},
            Gpio::F => {},
        }
    }
}

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

/// register structure for Gpio A module registers; need C representation for correct memory layout
#[repr(C)]
pub struct GpioAB {
    moder: Moder,
    otyper: Otyper,
    ospeedr: Ospeedr,
    pupdr: Pupdr,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    lckr: Lckr,
    afrl: Afrl,
    afrh: Afrh,
    brr: Brr,
}

create_regstruct! {[
Moder,
Otyper,
Ospeedr,
Pupdr,
Idr,
Odr,
Bsrr,
Lckr,
Afrl,
Afrh,
Brr
]}

impl GpioAB {
    /// method to the instance of Gpio A
    pub fn inst_a() -> &'static mut GpioAB {
        unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }
    }
    // create setter methods for all registers that allow write access
    // setters allow oring in a mask to the register
    create_setters! {[
    moder_set, Moder, moder ,
    otyper_set, Otyper, otyper,
    ospseedr_set,Ospeedr, ospeedr,
    pupdr_set, Pupdr, pupdr,
    odr_set, Odr, odr,
    bsrr_set, Bsrr, bsrr,
    lckr_set, Lckr, lckr,
    afrl_set, Afrl, afrl,
    afrh_set, Afrh, afrh,
    brr_set, Brr, brr
    ]}

    // create clears methods for all registers that allow write access
    // clears allow clearing register bits using the given mask
    create_clears! {[
    moder_clr, Moder, moder ,
    otyper_clr, Otyper, otyper,
    ospseedr_clr, Ospeedr, ospeedr,
    pupdr_clr, Pupdr, pupdr,
    odr_clr, Odr, odr,
    bsrr_clr, Bsrr, bsrr,
    lckr_clr, Lckr, lckr,
    afrl_clr, Afrl, afrl,
    afrh_clr, Afrh, afrh,
    brr_clr, Brr, brr
    ]}

    // create getters methods for all registers that allow read access
    // reads the value that is stored in the register; the value is
    // return as unnamed struct containing an u32 (access via varname.0)
    create_getters! {[
    moder_get, Moder, moder ,
    otyper_get, Otyper, otyper,
    ospseedr_get, Ospeedr, ospeedr,
    pupdr_get, Pupdr, pupdr,
    idr_get, Idr, idr,
    odr_get, Odr, odr,
    bsrr_get, Bsrr, bsrr,
    lckr_get, Lckr, lckr,
    afrl_get, Afrl, afrl,
    afrh_get, Afrh, afrh
    ]}
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

pub fn init() {
   //enable clock of  io port A peripheral instance
   set_reg_bitmsk(&mut Rcc::inst().ahbenr, 1 << 17);

   //define gpioa pin 5 as output
   GpioAB::inst_a().moder_set(Moder::comb(&[_MODER5_OUTPUT])); 
}
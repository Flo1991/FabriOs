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
use crate::mcal::rcc::Rcc;
use crate::mcal::util::{
    clr_reg_bitmsk, create_ro_regstruct, create_rw_regstruct, create_wo_regstruct, get_reg,
    modify_reg_bits, set_reg, set_reg_bitmsk,
};
//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------

/*
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
pub const _ODR_PIN6: Odr = Odr(1 << 6);
pub const _ODR_PIN7: Odr = Odr(1 << 7);
pub const _ODR_PIN8: Odr = Odr(1 << 8);
pub const _ODR_PIN9: Odr = Odr(1 << 9);
pub const _ODR_PIN10: Odr = Odr(1 << 10);
pub const _ODR_PIN11: Odr = Odr(1 << 11);
pub const _ODR_PIN12: Odr = Odr(1 << 12);
pub const _ODR_PIN13: Odr = Odr(1 << 13);
pub const _ODR_PIN14: Odr = Odr(1 << 14);
pub const _ODR_PIN15: Odr = Odr(1 << 15);
*/

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

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------
#[allow(dead_code)]
enum Gpio{
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}

#[allow(dead_code)]
enum PinNo {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
    P8 = 8,
    P9 = 9,
    P10 = 10,
    P11 = 11,
    P12 = 12,
    P13 = 13,
    P14 = 14,
    P15 = 15,
}




#[allow(dead_code)]
enum GpioType {
    AB(&'static mut GpioAB),
    CDEF(&'static mut GpioCDEF),
}

#[allow(dead_code)]
pub enum Mode {
    Input = 0,
    Output = 1,
    Alternate = 2,
    Analog = 3,
}

#[allow(dead_code)]
pub enum Otype {
    PushPull = 0,
    OpenDrain = 1,
}

#[allow(dead_code)]
pub enum Ospeed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

#[allow(dead_code)]
pub enum Pupd {
    NoPullUpPullDown = 0,
    PullUp = 1,
    PullDown = 2,
}

#[allow(dead_code)]
pub enum Af {
    AlternateFunc0 = 0,
    AlternateFunc1 = 1,
    AlternateFunc2 = 2,
    AlternateFunc3 = 3,
    AlternateFunc4 = 4,
    AlternateFunc5 = 5,
    AlternateFunc6 = 6,
    AlternateFunc7 = 7,
    AlternateFunc8 = 8,
    AlternateFunc9 = 9,
    AlternateFunc10 = 10,
    AlternateFunc11 = 11,
    AlternateFunc12 = 12,
    AlternateFunc13 = 13,
    AlternateFunc14 = 14,
    AlternateFunc15 = 15,
}

///have LQFP64 package for mcu
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Pin{
    A0,  //LQFP64 Pin 14
    A1,  //LQFP64 Pin 15
    A2,  //LQFP64 Pin 16
    A3,  //LQFP64 Pin 17
    A4,  //LQFP64 Pin 20
    A5,  //LQFP64 Pin 21
    A6,  //LQFP64 Pin 22
    A7,  //LQFP64 Pin 23
    A8,  //LQFP64 Pin 41
    A9,  //LQFP64 Pin 42
    A10, //LQFP64 Pin 43
    A11, //LQFP64 Pin 44
    A12, //LQFP64 Pin 45
    A13, //LQFP64 Pin 46
    A14, //LQFP64 Pin 49
    A15, //LQFP64 Pin 50

    B0,  //LQFP64 Pin 26
    B1,  //LQFP64 Pin 27
    B2,  //LQFP64 Pin 28
    B3,  //LQFP64 Pin 55
    B4,  //LQFP64 Pin 56
    B5,  //LQFP64 Pin 57
    B6,  //LQFP64 Pin 58
    B7,  //LQFP64 Pin 59

    B8,  //LQFP64 Pin 61
    B9,  //LQFP64 Pin 62
    B10, //LQFP64 Pin 29
    B11, //LQFP64 Pin 30
    B12, //LQFP64 Pin 33
    B13, //LQFP64 Pin 34
    B14, //LQFP64 Pin 35
    B15, //LQFP64 Pin 36

    C0,  //LQFP64 Pin 8
    C1,  //LQFP64 Pin 9
    C2,  //LQFP64 Pin 10
    C3,  //LQFP64 Pin 11
    C4,  //LQFP64 Pin 24
    C5,  //LQFP64 Pin 25
    C6,  //LQFP64 Pin 37
    C7,  //LQFP64 Pin 38
    C8,  //LQFP64 Pin 39
    C9,  //LQFP64 Pin 40
    C10, //LQFP64 Pin 51
    C11, //LQFP64 Pin 52
    C12, //LQFP64 Pin 53
    C13, //LQFP64 Pin 2
    C14, //LQFP64 Pin 3 - limited gpio, is OSC32_IN
    C15, //LQFP64 Pin 4 - limited gpio, is OSC32_OUT

    D2,  //LQFP64 Pin 54

    F0,  //LQFP64 Pin 5 - limited gpio, is OSC_IN
    F1,  //LQFP64 Pin 6 - limited gpio, is OSC_OUT
    F11, //LQFP64 Pin 60 - limited gpio, BOOT0 -> boot memory selection

}

impl Pin {
    const fn get_io_set(self) -> (Gpio, GpioType, PinNo) {
        match self {
            Pin::A0  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P0 ), 
            Pin::A1  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P1 ),  
            Pin::A2  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P2 ),  
            Pin::A3  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P3 ),  
            Pin::A4  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P4 ),  
            Pin::A5  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P5 ),  
            Pin::A6  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P6 ),  
            Pin::A7  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P7 ),  
            Pin::A8  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P8 ),  
            Pin::A9  => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P9 ),  
            Pin::A10 => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P10),
            Pin::A11 => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P11),
            Pin::A12 => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P12),
            Pin::A13 => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P13),
            Pin::A14 => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P14),
            Pin::A15 => (Gpio::A, GpioType::AB(unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }), PinNo::P15),
            Pin::B0  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P0 ), 
            Pin::B1  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P1 ), 
            Pin::B2  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P2 ), 
            Pin::B3  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P3 ), 
            Pin::B4  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P4 ), 
            Pin::B5  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P5 ), 
            Pin::B6  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P6 ), 
            Pin::B7  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P7 ), 
            Pin::B8  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P8 ), 
            Pin::B9  => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P9 ), 
            Pin::B10 => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P10), 
            Pin::B11 => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P11), 
            Pin::B12 => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P12), 
            Pin::B13 => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P13), 
            Pin::B14 => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P14), 
            Pin::B15 => (Gpio::B, GpioType::AB(unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }), PinNo::P15), 
            Pin::C0  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P0 ), 
            Pin::C1  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P1 ), 
            Pin::C2  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P2 ), 
            Pin::C3  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P3 ), 
            Pin::C4  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P4 ), 
            Pin::C5  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P5 ), 
            Pin::C6  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P6 ), 
            Pin::C7  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P7 ), 
            Pin::C8  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P8 ), 
            Pin::C9  => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P9 ), 
            Pin::C10 => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P10), 
            Pin::C11 => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P11), 
            Pin::C12 => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P12), 
            Pin::C13 => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P13), 
            Pin::C14 => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P14), 
            Pin::C15 => (Gpio::C, GpioType::CDEF(unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }), PinNo::P15), 
            Pin::D2  => (Gpio::D, GpioType::CDEF(unsafe { &mut *((Gpio::D.base()) as *mut GpioCDEF) }), PinNo::P2),  
            Pin::F0  => (Gpio::F, GpioType::CDEF(unsafe { &mut *((Gpio::F.base()) as *mut GpioCDEF) }), PinNo::P0),
            Pin::F1  => (Gpio::F, GpioType::CDEF(unsafe { &mut *((Gpio::F.base()) as *mut GpioCDEF) }), PinNo::P1),
            Pin::F11 => (Gpio::F, GpioType::CDEF(unsafe { &mut *((Gpio::F.base()) as *mut GpioCDEF) }), PinNo::P11),       
        }
    }
    
    #[allow(dead_code)]
    pub fn set(self) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let odr: Odr = Odr(1 << (io_pair.2 as u32));
        match io_pair.1 {
            GpioType::AB(base) => {
                base.odr.set_msk(odr);
            }
            GpioType::CDEF(base) => {
                base.odr.set_msk(odr);
            }
        }
    }

    #[allow(dead_code)]
    pub fn clr(self) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let odr: Odr = Odr(1 << (io_pair.2 as u32));
        match io_pair.1 {
            GpioType::AB(base) => {
                base.odr.clr_msk(odr);
            }
            GpioType::CDEF(base) => {
                base.odr.clr_msk(odr);
            }
        }
    }

    const fn create_mult(pins: &[Self]) -> Odr {
        let io_pair: (Gpio, _, PinNo) = pins[0].get_io_set();
        let mut odr: Odr = Odr(1 << (io_pair.2 as u32));
        let mut i = 0;
        while i < pins.len() {
            let next_io_pair: (Gpio, _, PinNo) = pins[i].get_io_set();
            let next_odr: Odr = Odr(1 << (next_io_pair.2 as u32));
            match io_pair.0 {
                Gpio::A => match next_io_pair.0 {
                    Gpio::A => {
                        odr.0 |= next_odr.0;
                    }
                    _ => {
                        panic!();
                    }
                },
                Gpio::B => match next_io_pair.0 {
                    Gpio::B => {
                        odr.0 |= next_odr.0;
                    }
                    _ => {
                        panic!();
                    }
                },
                Gpio::C => match next_io_pair.0 {
                    Gpio::C => {
                        odr.0 |= next_odr.0;
                    }
                    _ => {
                        panic!();
                    }
                },
                Gpio::D => match next_io_pair.0 {
                    Gpio::D => {
                        odr.0 |= next_odr.0;
                    }
                    _ => {
                        panic!();
                    }
                },
                Gpio::E => match next_io_pair.0 {
                    Gpio::E => {
                        odr.0 |= next_odr.0;
                    }
                    _ => {
                        panic!();
                    }
                },
                Gpio::F => match next_io_pair.0 {
                    Gpio::F => {
                        odr.0 |= next_odr.0;
                    }
                    _ => {
                        panic!();
                    }
                },
            }
            i += 1;
        }
        odr
    }

    #[allow(dead_code)]
    pub fn setmult(pins: &[Self]) {
        let io_pair: (Gpio, GpioType, PinNo) = pins[0].get_io_set();
        let regval = Self::create_mult(pins);
        match io_pair.1 {
            GpioType::AB(base) => {
                base.odr.set_msk(regval);
            }
            GpioType::CDEF(base) => {
                base.odr.set_msk(regval);
            }
        }
    }

    #[allow(dead_code)]
    pub fn clrmult(pins: &[Self]) {
        let io_pair: (Gpio, GpioType, PinNo) = pins[0].get_io_set();
        let regval = Self::create_mult(pins);
        match io_pair.1 {
            GpioType::AB(base) => {
                base.odr.clr_msk(regval);
            }
            GpioType::CDEF(base) => {
                base.odr.clr_msk(regval);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get(self) -> bool {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let idr_msk: Idr = Idr(io_pair.2 as u32);
        match io_pair.1 {
            GpioType::AB(base) => idr_msk.0 & base.idr.get().0 != 0,
            GpioType::CDEF(base) => idr_msk.0 & base.idr.get().0 != 0,
        }
    }

    #[allow(dead_code)]
    pub fn cfg_mode(self, mode: Mode) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let bit_pos: u32 = (io_pair.2 as u32) << 1;
        let regval: Moder = Moder((mode as u32) << bit_pos);
        let mod_mask: Moder = Moder((0x3) << bit_pos);
        match io_pair.1 {
            GpioType::AB(base) => {
                base.moder.modify(mod_mask, regval);
            }
            GpioType::CDEF(base) => {
                base.moder.modify(mod_mask, regval);
            }
        }
    }

    #[allow(dead_code)]
    pub fn cfg_otype(self, otype: Otype) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let bit_pos: u32 = io_pair.2 as u32;
        let regval: Otyper = Otyper((otype as u32) << bit_pos);
        let mod_mask: Otyper = Otyper((0x1) << bit_pos);
        match io_pair.1 {
            GpioType::AB(base) => {
                base.otyper.modify(mod_mask, regval);
            }
            GpioType::CDEF(base) => {
                base.otyper.modify(mod_mask, regval);
            }
        }
    }

    #[allow(dead_code)]
    pub fn cfg_ospeed(self, ospeed: Ospeed) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let bit_pos: u32 = (io_pair.2 as u32) << 1;
        let regval: Ospeedr = Ospeedr((ospeed as u32) << bit_pos);
        let mod_mask: Ospeedr = Ospeedr((0x3) << bit_pos);
        match io_pair.1 {
            GpioType::AB(base) => {
                base.ospeedr.modify(mod_mask, regval);
            }
            GpioType::CDEF(base) => {
                base.ospeedr.modify(mod_mask, regval);
            }
        }
    }

    #[allow(dead_code)]
    pub fn cfg_pupd(self, pupd: Pupd) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let bit_pos: u32 = (io_pair.2 as u32) << 1;
        let regval: Pupdr = Pupdr((pupd as u32) << bit_pos);
        let mod_mask: Pupdr = Pupdr((0x3) << bit_pos);
        match io_pair.1 {
            GpioType::AB(base) => {
                base.pupdr.modify(mod_mask, regval);
            }
            GpioType::CDEF(base) => {
                base.pupdr.modify(mod_mask, regval);
            }
        }
    }

    #[allow(dead_code)]
    pub fn cfg_af(self, af: Af) {
        let io_pair: (_, GpioType, PinNo) = self.get_io_set();
        let bit_no: u32 = io_pair.2 as u32;
        if bit_no < (PinNo::P8 as u32) {
            let bit_pos: u32 = bit_no << 2;
            let regval: Afrl = Afrl((af as u32) << bit_pos);
            let mod_mask: Afrl = Afrl((0xF) << bit_pos);
            match io_pair.1 {
                GpioType::AB(base) => {
                    base.afrl.modify(mod_mask, regval);
                }
                GpioType::CDEF(base) => {
                    base.afrl.modify(mod_mask, regval);
                }
            }
        } else {
            let bit_pos: u32 = (bit_no - PinNo::P8 as u32) << 2;
            let regval: Afrh = Afrh((af as u32) << bit_pos);
            let mod_mask: Afrh = Afrh((0xF) << bit_pos);
            match io_pair.1 {
                GpioType::AB(base) => {
                    base.afrh.modify(mod_mask, regval);
                }
                GpioType::CDEF(_base) => {
                    panic!();
                }
            }
        }
    }

}

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

/// register structure for Gpio A and B module registers; need C representation for correct memory layout
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

/// register structure for Gpio C - F module registers; need C representation for correct memory layout
#[repr(C)]
pub struct GpioCDEF {
    moder: Moder,
    otyper: Otyper,
    ospeedr: Ospeedr,
    pupdr: Pupdr,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    reserved0: u32,
    afrl: Afrl,
    afrh: Afrh,
    brr: Brr,
}



// create structs and macros for register handling - read / write registers
create_rw_regstruct! {[
Moder,
Otyper,
Ospeedr,
Pupdr,
Odr,
Lckr,
Afrl,
Afrh
]}

// create structs and macros for register handling - read only registers
create_ro_regstruct! {[
    Idr
]}

// create structs and macros for register handling - write only registers
create_wo_regstruct! {[
    Bsrr,
    Brr
]}

pub struct Lck(pub u32);

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------

impl GpioAB {
    /// method to the instance of Gpio A
    #[allow(dead_code)]
    fn inst_a() -> &'static mut GpioAB {
        unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }
    }
    /// method to the instance of Gpio B
    #[allow(dead_code)]
    fn inst_b() -> &'static mut GpioAB {
        unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }
    }
}

impl GpioCDEF {
    /// method to the instance of Gpio C
    #[allow(dead_code)]
    fn inst_c() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }
    }
    /// method to the instance of Gpio D
    #[allow(dead_code)]
    fn inst_d() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::D.base()) as *mut GpioCDEF) }
    }
    /// method to the instance of Gpio E
    #[allow(dead_code)]
    fn inst_e() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::E.base()) as *mut GpioCDEF) }
    }
    /// method to the instance of Gpio F
    #[allow(dead_code)]
    fn inst_f() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::F.base()) as *mut GpioCDEF) }
    }
}

impl Gpio {
    #[allow(dead_code)]
    pub fn write_lckr(self, lock_reg_val: Lck) {
        let regval = Lckr(lock_reg_val.0);
        match self {
            Self::A => GpioAB::inst_a().lckr.set(Lckr(regval.0 & 0x0001_FFFF)),
            Self::B => GpioAB::inst_b().lckr.set(Lckr(regval.0 & 0x0001_FFFF)),
            _ => panic!(), //only AB have lckr
        }
    }

    #[allow(dead_code)]
    pub fn read_lckr(self) -> Lck {
        match self {
            Self::A => Lck(GpioAB::inst_a().lckr.get().0),
            Self::B => Lck(GpioAB::inst_b().lckr.get().0),
            _ => panic!(), //only AB have lckr
    }
}
}

pub fn init() {
   //enable clock of  io port A-F peripheral instance
   set_reg_bitmsk(&mut Rcc::inst().ahbenr, 0x007E_0000);

   Pin::A5.cfg_mode(Mode::Output);
   //define gpioa pin 5 as output
   //GpioAB::inst_a().moder_set(Moder::comb(&[_MODER5_OUTPUT])); 
}
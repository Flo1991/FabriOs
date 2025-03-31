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
use crate::common::util::{clr_reg_bitmsk, get_reg, set_reg, set_reg_bitmsk, create_setters ,create_clears, create_readers, create_writers, create_regstruct};
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
    const fn obtain_odr (self) -> (Gpio, Odr) {
        match self {
            Pin::A0  => (Gpio::A, _ODR_PIN0 ), 
            Pin::A1  => (Gpio::A, _ODR_PIN1 ),  
            Pin::A2  => (Gpio::A, _ODR_PIN2 ),  
            Pin::A3  => (Gpio::A, _ODR_PIN3 ),  
            Pin::A4  => (Gpio::A, _ODR_PIN4 ),  
            Pin::A5  => (Gpio::A, _ODR_PIN5 ),  
            Pin::A6  => (Gpio::A, _ODR_PIN6 ),  
            Pin::A7  => (Gpio::A, _ODR_PIN7 ),  
            Pin::A8  => (Gpio::A, _ODR_PIN8 ),  
            Pin::A9  => (Gpio::A, _ODR_PIN9 ),  
            Pin::A10 => (Gpio::A, _ODR_PIN10),
            Pin::A11 => (Gpio::A, _ODR_PIN11),
            Pin::A12 => (Gpio::A, _ODR_PIN12),
            Pin::A13 => (Gpio::A, _ODR_PIN13),
            Pin::A14 => (Gpio::A, _ODR_PIN14),
            Pin::A15 => (Gpio::A, _ODR_PIN15),
            Pin::B0  => (Gpio::B, _ODR_PIN0 ), 
            Pin::B1  => (Gpio::B, _ODR_PIN1 ), 
            Pin::B2  => (Gpio::B, _ODR_PIN2 ), 
            Pin::B3  => (Gpio::B, _ODR_PIN3 ), 
            Pin::B4  => (Gpio::B, _ODR_PIN4 ), 
            Pin::B5  => (Gpio::B, _ODR_PIN5 ), 
            Pin::B6  => (Gpio::B, _ODR_PIN6 ), 
            Pin::B7  => (Gpio::B, _ODR_PIN7 ), 
            Pin::B8  => (Gpio::B, _ODR_PIN8 ), 
            Pin::B9  => (Gpio::B, _ODR_PIN9 ), 
            Pin::B10 => (Gpio::B, _ODR_PIN10), 
            Pin::B11 => (Gpio::B, _ODR_PIN11), 
            Pin::B12 => (Gpio::B, _ODR_PIN12), 
            Pin::B13 => (Gpio::B, _ODR_PIN13), 
            Pin::B14 => (Gpio::B, _ODR_PIN14), 
            Pin::B15 => (Gpio::B, _ODR_PIN15), 
            Pin::C0  => (Gpio::C, _ODR_PIN0 ), 
            Pin::C1  => (Gpio::C, _ODR_PIN1 ), 
            Pin::C2  => (Gpio::C, _ODR_PIN2 ), 
            Pin::C3  => (Gpio::C, _ODR_PIN3 ), 
            Pin::C4  => (Gpio::C, _ODR_PIN4 ), 
            Pin::C5  => (Gpio::C, _ODR_PIN5 ), 
            Pin::C6  => (Gpio::C, _ODR_PIN6 ), 
            Pin::C7  => (Gpio::C, _ODR_PIN7 ), 
            Pin::C8  => (Gpio::C, _ODR_PIN8 ), 
            Pin::C9  => (Gpio::C, _ODR_PIN9 ), 
            Pin::C10 => (Gpio::C, _ODR_PIN10), 
            Pin::C11 => (Gpio::C, _ODR_PIN11), 
            Pin::C12 => (Gpio::C, _ODR_PIN12), 
            Pin::C13 => (Gpio::C, _ODR_PIN13), 
            Pin::C14 => (Gpio::C, _ODR_PIN14), 
            Pin::C15 => (Gpio::C, _ODR_PIN15), 
            Pin::D2  => (Gpio::D, _ODR_PIN2),  
            Pin::F0  => (Gpio::F, _ODR_PIN0),
            Pin::F1  => (Gpio::F, _ODR_PIN1),
            Pin::F11 => (Gpio::F, _ODR_PIN11),       
        }
    }
    #[allow(dead_code)]
    pub fn set(self) {
        let io_pair: (Gpio, Odr) = self.obtain_odr();
        match io_pair.0 {
            Gpio::A => {
                GpioAB::inst_a().odr_set(io_pair.1);
            },
            Gpio::B => {
                GpioAB::inst_b().odr_set(io_pair.1);
            },
            Gpio::C => {
                GpioCDEF::inst_c().odr_set(io_pair.1);
            },
            Gpio::D => {
                GpioCDEF::inst_d().odr_set(io_pair.1);
            },
            Gpio::E => {
                GpioCDEF::inst_e().odr_set(io_pair.1);
            },
            Gpio::F => {
                GpioCDEF::inst_f().odr_set(io_pair.1);
            },
        }
    }
    #[allow(dead_code)]
    pub fn clr(self) {
        let io_pair: (Gpio, Odr) = self.obtain_odr();
        match io_pair.0 {
            Gpio::A => {
                GpioAB::inst_a().odr_clr(io_pair.1);
            },
            Gpio::B => {
                GpioAB::inst_b().odr_clr(io_pair.1);
            },
            Gpio::C => {
                GpioCDEF::inst_c().odr_clr(io_pair.1);
            },
            Gpio::D => {
                GpioCDEF::inst_d().odr_clr(io_pair.1);
            },
            Gpio::E => {
                GpioCDEF::inst_e().odr_clr(io_pair.1);
            },
            Gpio::F => {
                GpioCDEF::inst_f().odr_clr(io_pair.1);
            },
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
    fn inst_a() -> &'static mut GpioAB {
        unsafe { &mut *((Gpio::A.base()) as *mut GpioAB) }
    }
    /// method to the instance of Gpio B
    fn inst_b() -> &'static mut GpioAB {
        unsafe { &mut *((Gpio::B.base()) as *mut GpioAB) }
    }


    // create setter methods for all registers that allow write access
    // setters allow oring in a mask to the register
    create_setters! {[
    moder_set, Moder, moder ,
    otyper_set, Otyper, otyper,
    ospseedr_set,Ospeedr, ospeedr,
    pupdr_set, Pupdr, pupdr,
    odr_set, Odr, odr,
    lckr_set, Lckr, lckr,
    afrl_set, Afrl, afrl,
    afrh_set, Afrh, afrh
    ]}

    // create clears methods for all registers that allow write access
    // clears allow clearing register bits using the given mask
    create_clears! {[
    moder_clr, Moder, moder ,
    otyper_clr, Otyper, otyper,
    ospseedr_clr, Ospeedr, ospeedr,
    pupdr_clr, Pupdr, pupdr,
    odr_clr, Odr, odr,
    lckr_clr, Lckr, lckr,
    afrl_clr, Afrl, afrl,
    afrh_clr, Afrh, afrh
    ]}

    // create read methods for all registers that allow read access
    // reads the value that is stored in the register; the value is
    // return as unnamed struct containing an u32 (access via varname.0)
    create_readers! {[
    moder_read, Moder, moder ,
    otyper_read, Otyper, otyper,
    ospseedr_read, Ospeedr, ospeedr,
    pupdr_read, Pupdr, pupdr,
    idr_read, Idr, idr,
    odr_read, Odr, odr,
    lckr_read, Lckr, lckr,
    afrl_read, Afrl, afrl,
    afrh_read, Afrh, afrh
    ]}

    // create writer methods for all registers that allow write access
    // writer methods overwrite the full register with the given value
    create_writers! {[
        moder_write, Moder, moder ,
        otyper_write, Otyper, otyper,
        ospseedr_write,Ospeedr, ospeedr,
        pupdr_write, Pupdr, pupdr,
        odr_write, Odr, odr,
        bsrr_write, Bsrr, bsrr,
        lckr_write, Lckr, lckr,
        afrl_write, Afrl, afrl,
        afrh_write, Afrh, afrh,
        brr_write, Brr, brr
        ]}
}

impl GpioCDEF {
    /// method to the instance of Gpio C
    fn inst_c() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::C.base()) as *mut GpioCDEF) }
    }
    /// method to the instance of Gpio D
    fn inst_d() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::D.base()) as *mut GpioCDEF) }
    }
    /// method to the instance of Gpio E
    fn inst_e() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::E.base()) as *mut GpioCDEF) }
    }
    /// method to the instance of Gpio F
    fn inst_f() -> &'static mut GpioCDEF {
        unsafe { &mut *((Gpio::F.base()) as *mut GpioCDEF) }
    }

    // create setter methods for all registers that allow write access
    // setters allow oring in a mask to the register
    create_setters! {[
    moder_set, Moder, moder ,
    otyper_set, Otyper, otyper,
    ospseedr_set,Ospeedr, ospeedr,
    pupdr_set, Pupdr, pupdr,
    odr_set, Odr, odr,
    afrl_set, Afrl, afrl,
    afrh_set, Afrh, afrh
    ]}

    // create clears methods for all registers that allow write access
    // clears allow clearing register bits using the given mask
    create_clears! {[
    moder_clr, Moder, moder ,
    otyper_clr, Otyper, otyper,
    ospseedr_clr, Ospeedr, ospeedr,
    pupdr_clr, Pupdr, pupdr,
    odr_clr, Odr, odr,
    afrl_clr, Afrl, afrl,
    afrh_clr, Afrh, afrh
    ]}

    // create read methods for all registers that allow read access
    // reads the value that is stored in the register; the value is
    // return as unnamed struct containing an u32 (access via varname.0)
    create_readers! {[
    moder_read, Moder, moder ,
    otyper_read, Otyper, otyper,
    ospseedr_read, Ospeedr, ospeedr,
    pupdr_read, Pupdr, pupdr,
    idr_read, Idr, idr,
    odr_read, Odr, odr,
    afrl_read, Afrl, afrl,
    afrh_read, Afrh, afrh
    ]}

    // create writer methods for all registers that allow write access
    // writer methods overwrite the full register with the given value
    create_writers! {[
        moder_write, Moder, moder ,
        otyper_write, Otyper, otyper,
        ospseedr_write,Ospeedr, ospeedr,
        pupdr_write, Pupdr, pupdr,
        odr_write, Odr, odr,
        bsrr_write, Bsrr, bsrr,
        afrl_write, Afrl, afrl,
        afrh_write, Afrh, afrh,
        brr_write, Brr, brr
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
   //enable clock of  io port A-F peripheral instance
   set_reg_bitmsk(&mut Rcc::inst().ahbenr, 0x007E_0000);

   //define gpioa pin 5 as output
   GpioAB::inst_a().moder_set(Moder::comb(&[_MODER5_OUTPUT])); 
}
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
//! File : isr.rs
//!
//! # Short description
//! Interrupt service routines module containing all interrupt relevant functionality
//!
//! # Detailed description
//! This module contains and creates the hardware entry point at power on. Several names in this module must never
//! be mangled, because the names must match the linker script for correct behavior.
//! The vector table is linked to the specified section - need to use c union representation to setup the
//! vector table correctly.
//! The purpose of this implementation is to use as less interrupts as possible, so implemented no isr handling
//! functions. If need an interrupt handler, implement it in this module!
//!
//!
//!

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use crate::application_main;
use core::ptr;

//---------------------------------------------------------------------------------------------------------------------
// Types
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Statics
//---------------------------------------------------------------------------------------------------------------------
//keep the actual symbol G_VECTOR_TABLE_UI32 (--> no mangle) and provide linker section
//-->the vector table must be located at correct flash position
#[unsafe(no_mangle)]
#[unsafe(link_section = ".isr_vector")]
pub static G_VECTOR_TABLE_UI32: [ISRVector; 48] = [
    ISRVector {
        //first element in interrupt vector table of cortex m is stack pointer start address;
        //the stack pointer is predecrementing, so locate at end of ram for example (address can be one byte out of ram, due to predecrement)
        reserved: 0x2000_8000,
    },
    ISRVector {
        //second element in interrupt vector table is the entry point for Mcu startup;
        //means this is the starting point after a reset where the Mcu starts to execute its first instruction;
        //if implemented in C the compiler and linker takes care of correct addressing (if written in assembly one has to know whether the core supports /
        //should either use Thumb mode or Arm mode, the last bit of the address is 1 for Thumb mode and 0 for Arm mode (so not used for addressing in fact)
        entry: application_reset_handler,
    },
    ISRVector {
        handler: default_isr_handler, //NMI
    },
    ISRVector {
        handler: default_isr_handler, //HardFault
    },
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector {
        handler: default_isr_handler, //SVCall
    },
    ISRVector { reserved: 0 }, //reserved
    ISRVector { reserved: 0 }, //reserved
    ISRVector {
        handler: default_isr_handler, //PendSV
    },
    ISRVector {
        handler: default_isr_handler, //SysTick
    },
    ISRVector {
        handler: default_isr_handler, //WWDG
    },
    ISRVector {
        handler: default_isr_handler, //PVD_VDDIO2
    },
    ISRVector {
        handler: default_isr_handler, //RTC
    },
    ISRVector {
        handler: default_isr_handler, //FLASH
    },
    ISRVector {
        handler: default_isr_handler, //RCC_CRS
    },
    ISRVector {
        handler: default_isr_handler, //EXTI0_1
    },
    ISRVector {
        handler: default_isr_handler, //EXTI2_3
    },
    ISRVector {
        handler: default_isr_handler, //EXTI4_15
    },
    ISRVector {
        handler: default_isr_handler, //TSC
    },
    ISRVector {
        handler: default_isr_handler, //DMA_CH1
    },
    ISRVector {
        handler: default_isr_handler, //DMA_CH2_3_DMA2_CH1_2
    },
    ISRVector {
        handler: default_isr_handler, //DMA_CH4_5_6_7_DMA2_CH3_4_5
    },
    ISRVector {
        handler: default_isr_handler, //ADC_COMP
    },
    ISRVector {
        handler: default_isr_handler, //TIM1_BRK_UP_TRG_COM
    },
    ISRVector {
        handler: default_isr_handler, //TIM1_CC
    },
    ISRVector {
        handler: default_isr_handler, //TIM2
    },
    ISRVector {
        handler: default_isr_handler, //TIM3
    },
    ISRVector {
        handler: default_isr_handler, //TIM6_DAC
    },
    ISRVector {
        handler: default_isr_handler, //TIM7
    },
    ISRVector {
        handler: default_isr_handler, //TIM14
    },
    ISRVector {
        handler: default_isr_handler, //TIM15
    },
    ISRVector {
        handler: default_isr_handler, //TIM16
    },
    ISRVector {
        handler: default_isr_handler, //TIM17
    },
    ISRVector {
        handler: default_isr_handler, //I2C1
    },
    ISRVector {
        handler: default_isr_handler, //I2C2
    },
    ISRVector {
        handler: default_isr_handler, //SPI1
    },
    ISRVector {
        handler: default_isr_handler, //SPI2
    },
    ISRVector {
        handler: default_isr_handler, //USART1
    },
    ISRVector {
        handler: default_isr_handler, //USART2
    },
    ISRVector {
        handler: default_isr_handler, //USART3_4_5_6_7_8
    },
    ISRVector {
        handler: default_isr_handler, //CEC_CAN
    },
    ISRVector {
        handler: default_isr_handler, //USB
    },
];

//---------------------------------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------------------------------
/// This define is used for stack pattern initialization; the reserve is the number of bytes from the start of stack
/// which will not be initialized to a default pattern; this must be done, because otherwise the init itself overwrites
/// stack memory which is currently in use
const STARTUP_STACK_RESERVE: usize = 128;

//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Unions
//---------------------------------------------------------------------------------------------------------------------

///use C representation of union, because need exact behavior to create the vector table;
///size will be usize and the abstraction allows creation of an array for vector table
#[repr(C)]
pub union ISRVector {
    entry: unsafe extern "C" fn() -> !,
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------

/// # Safety
///
/// This function is the program entry point; must match to linker script! Must match / be part of G_VECTOR_TABLE_UI32!
/// The user has to ensure correctness!
/// The function name must not be mangled, so that the symbol name stays like it is, so that it can be linked correctly,
/// because this is the entry point at Mcu startup
#[unsafe(no_mangle)]
pub unsafe extern "C" fn application_reset_handler() -> ! {
    //need this extern definitions to get setup the mcu memory regarding the linker script
    unsafe extern "C" {
        unsafe static _data_size: usize;
        unsafe static mut _data_loadaddr: u8;
        unsafe static mut _sdata: u8;

        unsafe static mut _sbss: u8;
        unsafe static _bss_size: usize;

        unsafe static mut _estack: u8;
        unsafe static _stack_size: usize;
    }

    //setup data section
    let count: usize = &_data_size as *const usize as usize;
    let load_addr: *const u8 = ptr::addr_of!(_data_loadaddr);
    let sdata: *mut u8 = &raw mut _sdata;
    ptr::copy_nonoverlapping(load_addr, sdata, count);

    //setup bss section
    let count: usize = &_bss_size as *const usize as usize;
    let sbss: *mut u8 = &raw mut _sbss;
    ptr::write_bytes(sbss, 0, count);

    //setup stack area
    let count: usize = &_stack_size as *const usize as usize - STARTUP_STACK_RESERVE;
    let estack: *mut u8 = &raw mut _estack;
    ptr::write_bytes(estack, 0xA8, count);

    //will not return from application_main!
    application_main();
}

/// # Safety
///
/// This function is the default interrupt service handler function
/// The user has to ensure correctness!
/// do not mangle the naming, because need this function to build the isr vector table
#[unsafe(no_mangle)]
pub unsafe extern "C" fn default_isr_handler() {
    let _x = 42;

    // can't return so we go into an infinite loop here
    #[allow(clippy::empty_loop)]
    loop {}
}

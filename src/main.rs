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
//! File : main.rs
//!
//! # Short description
//! This is the main module file from which the crate binary is created.
//!
//! # Detailed description
//! The fabriOs project is a basic rust Os starting point using no unstable rust features and focusing only on
//! core implementations. This limitation is done to allow the creation of applications that can easily be compiled
//! by a safety certified compiler to achieve a certified application. One can estimate that a certified core library
//! is realistic, while a rust std lib certification will be very hard.
//! For certification reasons the application is completely written in rust with using inline assembly, to avoid
//! the dependency to an external compiler (e.g. if adapt and link C code).
//!
//! The scheduling scheme is cooperative and interrupt free. This maximizes the applications reproduceability.
//! The implementation is simple in order to allow simple porting to other mcu architectures.
//!
//! The project is work in progress!
//!
//!

#![no_main]
#![no_std]

//---------------------------------------------------------------------------------------------------------------------
// Includes
//---------------------------------------------------------------------------------------------------------------------
use core::arch::asm;
use core::panic::PanicInfo;

mod appl;
mod common;
mod mcal;
mod rte;
mod servl;

use common::util::set_reg_bitmsk;
use mcal::gpio::GpioAB;
use mcal::gpt::Timer6_7;
use mcal::rcc::Rcc;
use servl::sched;

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

//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------

/// blocking delay function
///
/// delay: u32 loop amounts to delay, 1 count should match 1us (currently does not due to overhead)
#[allow(dead_code)]
fn delay_wait_us(delay: u32) {
    for _i in 0..delay {
        unsafe {
            asm!(".rept 3  ; \r\n", "nop       ; \r\n", ".endr  ; \r\n");
        }
    }
}

/// function to set the Led on
fn app_set_led_on() {
    set_reg_bitmsk(&mut GpioAB::inst_a().odr, 1 << 5);
}

//need to define a panic handler; will get here in case of panic; ends in infinity loop
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

/// main function; is never left in normal operation
fn application_main() -> ! {
    let _x = 42;

    //enable clock of  io port A peripheral instance
    set_reg_bitmsk(&mut Rcc::inst().ahbenr, 1 << 17);

    //define gpioa pin 5 as output
    set_reg_bitmsk(&mut GpioAB::inst_a().moder, 1 << (5 << 1));

    Timer6_7::inst_6().init();

    crate::servl::sched::s_init();

    sched::sched_run();

    // can't return so we go into an infinite loop here; if we reach here, had bad
    // error during scheduling
    #[allow(clippy::empty_loop)]
    loop {
        app_set_led_on();
    }

    //should never get here
}

//! STM32 microcontroller
//!
//! This crate provides functionality common to STM32 microcontrollers
//!
//! NOTE: Currently, only the STM32F100RB microcontroller is supported
//!
//! - Interrupt vector
//!   - All the interrupts are weakly linked to the `abort` function provided by the `cortex`
//!     crate.
//! - Register definition of peripherals
//! - A `start` function. This function resets all the peripherals and then calls the external
//!   `main` function. If the `main` function exits, then the `abort` function is called.

#![feature(core)]
#![feature(no_std)]
#![no_std]

extern crate core;
extern crate cortex;
extern crate volatile;

#[macro_use]
extern crate reg;

pub mod gpio;
pub mod interrupt;
pub mod peripheral;
pub mod rcc;
pub mod tim;

fn reset_peripherals() {
    let rcc = peripheral::rcc();

    let initial = rcc.apb2rstr.get();
    rcc.apb2rstr.set({
        use rcc::apb2rstr::prelude::*;

        AFIORST | IOPARST | IOPBRST | IOPCRST | IOPDRST | IOPERST | IOPFRST | IOPGRST | ADC1RST |
            TIM1RST | SPI1RST | USART1RST | TIM15RST | TIM16RST | TIM17RST
    });

    rcc.apb2rstr.set({
        initial
    });

    let initial = rcc.apb1rstr.get();
    rcc.apb1rstr.set({
        use rcc::apb1rstr::prelude::*;

        TIM2RST | TIM3RST | TIM4RST | TIM5RST | TIM6RST | TIM7RST | TIM12RST | TIM13RST |
            TIM14RST | WWDGRST | SPI2RST | SPI3RST | USART2RST | USART3RST | UART4RST | UART5RST |
            I2C1RST | I2C2RST | BKPRST | DACRST | CECRST
    });

    rcc.apb1rstr.set({
        initial
    });
}

#[no_mangle]
pub unsafe extern fn start() {
    extern {
        fn main();
    }

    reset_peripherals();

    main();

    cortex::abort();
}

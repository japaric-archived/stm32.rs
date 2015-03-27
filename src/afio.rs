//! Alternate function I/O and debug configuration

use volatile::RW;

#[repr(C)]
pub struct Afio {
    /* 0x00 */ evcr: RW<u32>,
    /* 0x04 */ mapr: RW<u32>,
    /* 0x08 */ exticr1: RW<u32>,
    /* 0x0C */ exticr2: RW<u32>,
    /* 0x10 */ exticr3: RW<u32>,
    /* 0x14 */ exticr4: RW<u32>,
    /* 0x18 */ _0: u32,
    /* 0x1C */ mapr2: RW<u32>,
}

//! General purpose and alternate function IO

pub mod crh;
pub mod crl;

use volatile::{RO, RW, WO};

#[repr(C)]
pub struct Gpio {
    /// Configuration register low
    /* 0x00 */ pub crl: RW<crl::Register>,
    /// Configuration register high
    /* 0x04 */ pub crh: RW<crh::Register>,
    /* 0x08 */ idr: RO<u32>,
    /* 0x0C */ odr: RW<u32>,
    /// Port bit set/reset register
    /* 0x10 */ pub bsrr: WO<bsrr::Register>,
    /* 0x14 */ brr: WO<u32>,
    /* 0x18 */ lckr: RW<u32>,
}

reg!(bsrr: u32 {
    BS0: 0,
    BS1: 1,
    BS2: 2,
    BS3: 3,
    BS4: 4,
    BS5: 5,
    BS6: 6,
    BS7: 7,
    BS8: 8,
    BS9: 9,
    BS10: 10,
    BS11: 11,
    BS12: 12,
    BS13: 13,
    BS14: 14,
    BS15: 15,
    BR0: 16,
    BR1: 17,
    BR2: 18,
    BR3: 19,
    BR4: 20,
    BR5: 21,
    BR6: 22,
    BR7: 23,
    BR8: 24,
    BR9: 25,
    BR10: 26,
    BR11: 27,
    BR12: 28,
    BR13: 29,
    BR14: 30,
    BR15: 31,
});

pub enum Drive {
    OpenDrain,
    PushPull,
}

pub enum Function {
    Alternate,
    GeneralPurpose,
}

pub enum Input {
    Analog,
    Floating,
    Pull,
}

pub enum Mode {
    Input(Input),
    Output(Function, Drive, Speed),
}

impl Mode {
    fn u32(&self) -> u32 {
        use self::Drive::*;
        use self::Function::*;
        use self::Input::*;
        use self::Mode::*;
        use self::Speed::*;

        match *self {
            Input(Analog) => 0,
            Input(Floating) => 4,
            Input(Pull) => 8,
            Output(Alternate, OpenDrain, _10MHz) => 13,
            Output(Alternate, OpenDrain, _2MHz) => 14,
            Output(Alternate, OpenDrain, _50MHz) => 15,
            Output(Alternate, PushPull, _10MHz) => 9,
            Output(Alternate, PushPull, _2MHz) => 10,
            Output(Alternate, PushPull, _50MHz) => 11,
            Output(GeneralPurpose, OpenDrain, _10MHz) => 5,
            Output(GeneralPurpose, OpenDrain, _2MHz) => 6,
            Output(GeneralPurpose, OpenDrain, _50MHz) => 7,
            Output(GeneralPurpose, PushPull, _10MHz) => 1,
            Output(GeneralPurpose, PushPull, _2MHz) => 2,
            Output(GeneralPurpose, PushPull, _50MHz) => 3,
        }
    }
}

pub enum Speed {
    _10MHz,
    _2MHz,
    _50MHz,
}

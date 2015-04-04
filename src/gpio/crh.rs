//! Configuration register high

pub mod prelude {
    pub use gpio::Drive::*;
    pub use gpio::Function::*;
    pub use gpio::Input::*;
    pub use gpio::Mode;
    pub use gpio::Speed::*;
    pub use gpio::crh::Pin;
}

const BITS: u8 = 4;
const MASK: u32 = 0b1111;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    pub fn configure(mut self, pin: Pin, mode: super::Mode) -> Register {
        let offset = BITS * pin.u8();
        self.0 &= !(MASK << offset);
        self.0 |= mode.u32() << offset;
        self
    }
}

pub enum Pin {
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
}

impl Pin {
    fn u8(&self) -> u8 {
        use self::Pin::*;

        match *self {
            _8 => 0,
            _9 => 1,
            _10 => 2,
            _11 => 3,
            _12 => 4,
            _13 => 5,
            _14 => 6,
            _15 => 7,
        }
    }
}

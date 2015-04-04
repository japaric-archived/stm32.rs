//! Configuration register low

pub mod prelude {
    pub use gpio::Drive::*;
    pub use gpio::Function::*;
    pub use gpio::Input::*;
    pub use gpio::Mode;
    pub use gpio::Speed::*;
    pub use gpio::crl::Pin;
}

const BITS: u8 = 4;
const MASK: u32 = 0b1111;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Configures a pin as input or output
    pub fn configure(mut self, pin: Pin, mode: super::Mode) -> Register {
        let offset = BITS * pin.u8();
        self.0 &= !(MASK << offset);
        self.0 |= mode.u32() << offset;
        self
    }
}

pub enum Pin {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl Pin {
    fn u8(&self) -> u8 {
        use self::Pin::*;

        match *self {
            _0 => 0,
            _1 => 1,
            _2 => 2,
            _3 => 3,
            _4 => 4,
            _5 => 5,
            _6 => 6,
            _7 => 7,
        }
    }
}

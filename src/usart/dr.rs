use volatile::Into;

#[derive(Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Returns the lower byte of the register
    pub fn u8(&self) -> u8 {
        self.0 as u8
    }
}

impl Into<Register> for u8 {
    fn convert_into(self) -> Register {
        Register(self as u32)
    }
}

use volatile::Into;

#[derive(Copy)]
#[repr(C)]
pub struct Register(u32);

impl Into<Register> for u8 {
    fn convert_into(self) -> Register {
        Register(self as u32)
    }
}

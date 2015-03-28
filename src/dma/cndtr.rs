use volatile::Into;

#[derive(Copy)]
#[repr(C)]
pub struct Register(u32);

impl Into<Register> for u16 {
    fn convert_into(self) -> Register {
        Register(self as u32)
    }
}

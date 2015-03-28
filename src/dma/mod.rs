use volatile::{RO, RW, WO};

pub mod cndtr;

#[repr(C)]
pub struct Dma {
    /// Interrupt status register
    /* 0x00 */ pub isr: RO<isr::Register>,
    /// Interrupt clear flag register
    /* 0x04 */ pub ifcr: WO<ifcr::Register>,
    /// Channel 1 configuration register
    /* 0x08 */ pub ccr1: RW<ccr::Register>,
    /// Channel 1 number of data register
    /* 0x0C */ pub cndtr1: RW<cndtr::Register>,
    /// Channel 1 peripheral address register
    /* 0x10 */ pub cpar1: RW<u32>,
    /// Channel 1 memory address register
    /* 0x14 */ pub cmar1: RW<u32>,
    /* 0x18 */ _0: u32,
    /// Channel 2 configuration register
    /* 0x1C */ pub ccr2: RW<ccr::Register>,
    /// Channel 2 number of data register
    /* 0x20 */ pub cndtr2: RW<cndtr::Register>,
    /// Channel 2 peripheral address register
    /* 0x24 */ pub cpar2: RW<u32>,
    /// Channel 2 memory address register
    /* 0x28 */ pub cmar2: RW<u32>,
    /* 0x2C */ _1: u32,
    /// Channel 3 configuration register
    /* 0x30 */ pub ccr3: RW<ccr::Register>,
    /// Channel 3 number of data register
    /* 0x34 */ pub cndtr3: RW<cndtr::Register>,
    /// Channel 3 peripheral address register
    /* 0x38 */ pub cpar3: RW<u32>,
    /// Channel 3 memory address register
    /* 0x3C */ pub cmar3: RW<u32>,
    /* 0x40 */ _2: u32,
    /// Channel 4 configuration register
    /* 0x44 */ pub ccr4: RW<ccr::Register>,
    /// Channel 4 number of data register
    /* 0x48 */ pub cndtr4: RW<cndtr::Register>,
    /// Channel 4 peripheral address register
    /* 0x4C */ pub cpar4: RW<u32>,
    /// Channel 4 memory address register
    /* 0x50 */ pub cmar4: RW<u32>,
    /* 0x54 */ _3: u32,
    /// Channel 5 configuration register
    /* 0x58 */ pub ccr5: RW<ccr::Register>,
    /// Channel 5 number of data register
    /* 0x5C */ pub cndtr5: RW<cndtr::Register>,
    /// Channel 5 peripheral address register
    /* 0x60 */ pub cpar5: RW<u32>,
    /// Channel 5 memory address register
    /* 0x64 */ pub cmar5: RW<u32>,
    /* 0x68 */ _4: u32,
    /// Channel 6 configuration register
    /* 0x6C */ pub ccr6: RW<ccr::Register>,
    /// Channel 6 number of data register
    /* 0x70 */ pub cndtr6: RW<cndtr::Register>,
    /// Channel 6 peripheral address register
    /* 0x74 */ pub cpar6: RW<u32>,
    /// Channel 6 memory address register
    /* 0x78 */ pub cmar6: RW<u32>,
    /* 0x7C */ _5: u32,
    /// Channel 7 configuration register
    /* 0x80 */ pub ccr7: RW<ccr::Register>,
    /// Channel 7 number of data register
    /* 0x84 */ pub cndtr7: RW<cndtr::Register>,
    /// Channel 7 peripheral address register
    /* 0x88 */ pub cpar7: RW<u32>,
    /// Channel 7 memory address register
    /* 0x8c */ pub cmar7: RW<u32>,
    /* 0x90 */ _6: u32,
}

reg!(isr: u32 {
    GIF1: 0,
    TCIF1: 1,
    HTIF1: 2,
    TEIF1: 3,
    GIF2: 4,
    TCIF2: 5,
    HTIF2: 6,
    TEIF2: 7,
    GIF3: 8,
    TCIF3: 9,
    HTIF3: 10,
    TEIF3: 11,
    GIF4: 12,
    TCIF4: 13,
    HTIF4: 14,
    TEIF4: 15,
    GIF5: 16,
    TCIF5: 17,
    HTIF5: 18,
    TEIF5: 19,
    GIF6: 20,
    TCIF6: 21,
    HTIF6: 22,
    TEIF6: 23,
    GIF7: 24,
    TCIF7: 25,
    HTIF7: 26,
    TEIF7: 27,
});

reg!(ifcr: u32 {
    CGIF1: 0,
    CTCIF1: 1,
    CHTIF1: 2,
    CTEIF1: 3,
    CGIF2: 4,
    CTCIF2: 5,
    CHTIF2: 6,
    CTEIF2: 7,
    CGIF3: 8,
    CTCIF3: 9,
    CHTIF3: 10,
    CTEIF3: 11,
    CGIF4: 12,
    CTCIF4: 13,
    CHTIF4: 14,
    CTEIF4: 15,
    CGIF5: 16,
    CTCIF5: 17,
    CHTIF5: 18,
    CTEIF5: 19,
    CGIF6: 20,
    CTCIF6: 21,
    CHTIF6: 22,
    CTEIF6: 23,
    CGIF7: 24,
    CTCIF7: 25,
    CHTIF7: 26,
    CTEIF7: 27,
});

reg!(ccr: u32 {
    bits {
        EN: 0,
        TCIE: 1,
        HTIE: 2,
        TEIE: 3,
        DIR: 4,
        CIRC: 5,
        PINC: 6,
        MINC: 7,
        MEM2MEM: 14,
    }
    bitfields {
        PSIZE: 8 {
            _8: 0b00,
            _16: 0b01,
            _32: 0b10,
        },
        MSIZE: 10 {
            _8: 0b00,
            _16: 0b01,
            _32: 0b10,
        },
        PL: 12 {
            Low: 0b00,
            Medium: 0b01,
            High: 0b10,
            VeryHigh: 0b11,
        }
    }
});

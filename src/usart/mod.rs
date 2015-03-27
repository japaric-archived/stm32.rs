//! Universal Synchronous Asynchronous Receiver Transmitter

#![allow(non_camel_case_types)]

pub mod dr;
pub mod brr;

use volatile::RW;

#[repr(C)]
pub struct Usart {
    /* 0x00 */ pub sr: RW<sr::Register>,
    /* 0x04 */ pub dr: RW<dr::Register>,
    /* 0x08 */ pub brr: RW<brr::Register>,
    /* 0x0C */ pub cr1: RW<cr1::Register>,
    /* 0x10 */ pub cr2: RW<cr2::Register>,
    /* 0x14 */ pub cr3: RW<cr3::Register>,
    /* 0x18 */ gtpr: RW<u32>,
}

reg!(sr: u32 {
    PE: 0,
    FE: 1,
    NE: 2,
    ORE: 3,
    IDLE: 4,
    RXNE: 5,
    TC: 6,
    TXE: 7,
    LBD: 8,
    CTS: 9,
});

reg!(cr1: u32 {
    SBK: 0,
    RWU: 1,
    RE: 2,
    TE: 3,
    IDLEIE: 4,
    RXNEIE: 5,
    TCIE: 6,
    TXEIE: 7,
    PEIE: 8,
    PS: 9,
    PCIE: 10,
    WAKE: 11,
    M: 12,
    UE: 13,
});

reg!(cr2: u32 {
    bits {
        LBDL: 6,
        LBDIE: 7,
        LBCL: 8,
        CPHA: 9,
        CPOL: 10,
        CLKEN: 11,
        LINEN: 14,
    }
    bitfields {
        ADD: 0 {
            _0: 0b0000,
            _1: 0b0001,
            _2: 0b0010,
            _3: 0b0011,
            _4: 0b0100,
            _5: 0b0101,
            _6: 0b0110,
            _7: 0b0111,
            _8: 0b1000,
            _9: 0b1001,
            _10: 0b1010,
            _11: 0b1011,
            _12: 0b1100,
            _13: 0b1101,
            _14: 0b1110,
            _15: 0b1111,
        },
        STOP: 12 {
            _1: 0b00,
            _0_5: 0b01,
            _2: 0b10,
            _1_5: 0b11,
        }
    }
});

reg!(cr3: u32 {
    EIE: 0,
    IREN: 1,
    IRLP: 2,
    HDSEL: 3,
    NACK: 4,
    SCEN: 5,
    DMAR: 6,
    DMAT: 7,
    RTSE: 8,
    CTSE: 9,
    CTSIE: 10,
});

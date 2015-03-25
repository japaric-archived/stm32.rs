use volatile::{RW, WO};

#[repr(C)]
pub struct Tim {
    /// TIM control register 1
    /* 0x00 */ pub cr1: RW<cr1::Register>,
    /* 0x02 */ _0: u16,
    /// TIM control register 2
    /* 0x04 */ cr2: RW<u16>,
    /* 0x06 */ _1: [u16; 3],
    /// TIM DMA/Interrupt enable register
    /* 0x0C */ pub dier: RW<dier::Register>,
    /* 0x0E */ _2: u16,
    /// TIM status register
    /* 0x10 */ pub sr: RW<sr::Register>,
    /* 0x12 */ _3: u16,
    /// TIM event generation
    /* 0x14 */ pub egr: WO<egr::Register>,
    /* 0x16 */ _4: [u16; 7],
    /// TIM counter
    /* 0x24 */ pub cnt: RW<u16>,
    /* 0x26 */ _5: u16,
    /// TIM prescaler
    /* 0x28 */ pub psc: RW<u16>,
    /* 0x2A */ _6: u16,
    /// TIM auto-reload
    /* 0x2C */ pub arr: RW<u16>,
}

reg!(cr1: u16 {
    CEN: 0,
    UDIS: 1,
    URS: 2,
    OPM: 3,
    ARPE: 7,
});

reg!(dier: u16 {
    UIE: 0,
    UDE: 8,
});

reg!(sr: u16 {
    UIF: 0,
});

reg!(egr: u16 {
    UG: 0,
});

//! Reset and clock control

use volatile::RW;

#[repr(C)]
pub struct Rcc {
    /// Clock control register
    /* 0x00 */ cr: RW<u32>,
    /// Clock configuration register
    /* 0x04 */ cfgr: RW<u32>,
    /// Clock interrupt register
    /* 0x08 */ cir: RW<u32>,
    /// APB2 peripheral reset register
    /* 0x0C */ pub apb2rstr: RW<apb2rstr::Register>,
    /// APB1 peripheral reset register
    /* 0x10 */ pub apb1rstr: RW<apb1rstr::Register>,
    /// AHB peripheral clock enable register
    /* 0x14 */ pub ahbenr: RW<ahbenr::Register>,
    /// APB2 peripheral clock enable register
    /* 0x18 */ pub apb2enr: RW<apb2enr::Register>,
    /// APB1 peripheral clock enable register
    /* 0x1C */ pub apb1enr: RW<apb1enr::Register>,
    /// Backup domain control register
    /* 0x20 */ bdcr: RW<u32>,
    /// Control/status register
    /* 0x24 */ csr: RW<u32>,
    /* 0x28 */ _0: u32,
    /// Clock configuration register 2
    /* 0x2C */ cfgr2: RW<u32>,
}

reg!(apb2rstr: u32 {
    AFIORST: 0,
    IOPARST: 2,
    IOPBRST: 3,
    IOPCRST: 4,
    IOPDRST: 5,
    IOPERST: 6,
    IOPFRST: 7,
    IOPGRST: 8,
    ADC1RST: 9,
    TIM1RST: 11,
    SPI1RST: 12,
    USART1RST: 14,
    TIM15RST: 16,
    TIM16RST: 17,
    TIM17RST: 18,
});

reg!(apb1rstr: u32 {
    TIM2RST: 0,
    TIM3RST: 1,
    TIM4RST: 2,
    TIM5RST: 3,
    TIM6RST: 4,
    TIM7RST: 5,
    TIM12RST: 6,
    TIM13RST: 7,
    TIM14RST: 8,
    WWDGRST: 11,
    SPI2RST: 14,
    SPI3RST: 15,
    USART2RST: 17,
    USART3RST: 18,
    UART4RST: 19,
    UART5RST: 20,
    I2C1RST: 21,
    I2C2RST: 22,
    BKPRST: 27,
    PWRRST: 28,
    DACRST: 29,
    CECRST: 30,
});

reg!(ahbenr: u32 {
    DMA1EN: 0,
    DMA2EN: 1,
    SRAMEN: 2,
    FLITFEN: 4,
    CRCEN: 6,
    FSMCEN: 8,
});

reg!(apb2enr: u32 {
    AFIOEN: 0,
    IOPAEN: 2,
    IOPBEN: 3,
    IOPCEN: 4,
    IOPDEN: 5,
    IOPEEN: 6,
    IOPFEN: 7,
    IOPGEN: 8,
    ADC1EN: 9,
    TIM1EN: 11,
    SPI1EN: 12,
    USART1EN: 14,
    TIM15EN: 16,
    TIM16EN: 17,
    TIM17EN: 18,
});

reg!(apb1enr: u32 {
    TIM2EN: 0,
    TIM3EN: 1,
    TIM4EN: 2,
    TIM5EN: 3,
    TIM6EN: 4,
    TIM7EN: 5,
    TIM12EN: 6,
    TIM13EN: 7,
    TIM14EN: 8,
    WWDGEN: 11,
    SPI2EN: 14,
    SPI3EN: 15,
    USART2EN: 17,
    USART3EN: 18,
    UART4EN: 19,
    UART5EN: 20,
    I2C1EN: 21,
    I2C2EN: 22,
    BKPEN: 27,
    PWREN: 28,
    DACEN: 29,
    CECEN: 30,
});

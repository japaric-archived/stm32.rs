use core::option::Option::{None, Some, self};

extern {
    /// Window Watchdog interrupt
    pub fn wwdg();
    /// PVD through EXTI Line detection interrupt
    pub fn pvd();
    /// Tamper and TimeStamp through EXTI line interrupt
    pub fn tamper_stamp();
    /// RTC Wakeup through EXTI line interrupt
    pub fn rtc_wkup();
    /// Flash global interrput
    pub fn flash();
    /// RCC global interrupt
    pub fn rcc();
    /// EXTI Line0 interrupt
    pub fn exti0();
    /// EXTI Line1 interrupt
    pub fn exti1();
    /// EXTI Line2 interrupt
    pub fn exti2();
    /// EXTI Line3 interrupt
    pub fn exti3();
    /// EXTI Line4 interrupt
    pub fn exti4();
    /// DMA1 Channel 1 global interrupt
    pub fn dma1_channel1();
    /// DMA1 Channel 2 global interrupt
    pub fn dma1_channel2();
    /// DMA1 Channel 3 global interrupt
    pub fn dma1_channel3();
    /// DMA1 Channel 4 global interrupt
    pub fn dma1_channel4();
    /// DMA1 Channel 5 global interrupt
    pub fn dma1_channel5();
    /// DMA1 Channel 6 global interrupt
    pub fn dma1_channel6();
    /// DMA1 Channel 7 global interrupt
    pub fn dma1_channel7();
    /// ADC1 global interrupt
    pub fn adc1();
    /// EXTI Line[9:5] interrupts
    pub fn exti9_5();
    /// TIM1 Break and TIM15 global interrupt
    pub fn tim1_brk_tim15();
    /// TIM1 Update and TIM16 global interrupts
    pub fn tim1_up_tim16();
    /// TIM1 Trigger and Communication and TIM17 global interrupts
    pub fn tim1_trg_com_tim17();
    /// TIM1 Capture Compare interrupt
    pub fn tim1_cc();
    /// TIM2 global interrupt
    pub fn tim2();
    /// TIM3 global interrupt
    pub fn tim3();
    /// TIM4 global interrupt
    pub fn tim4();
    /// I2C1 event interrupt
    pub fn i2c1_ev();
    /// I2C1 error interrupt
    pub fn i2c1_er();
    /// I2C2 event interrupt
    pub fn i2c2_ev();
    /// I2C2 error interrupt
    pub fn i2c2_er();
    /// SPI1 global interrupt
    pub fn spi1();
    /// SPI2 global interrupt
    pub fn spi2();
    /// USART1 global interrupt
    pub fn usart1();
    /// USART2 global interrupt
    pub fn usart2();
    /// USART3 global interrupt
    pub fn usart3();
    /// EXTI Line[15:10] interrupts
    pub fn exti15_10();
    /// RTC Alarms (A and B) through EXTI line interrupt
    pub fn rtc_alarm();
    /// CEC global interrupt
    pub fn cec();
    /// TIM12 global interrupt
    pub fn tim12();
    /// TIM13 global interrupt
    pub fn tim13();
    /// TIM14 global interrupt
    pub fn tim14();
    /// FSMC global interrupt
    pub fn fsmc();
    /// TIM5 global interrupt
    pub fn tim5();
    /// SPI3 global interrupt
    pub fn spi3();
    /// UART4 global interrupt
    pub fn uart4();
    /// UART5 global interrupt
    pub fn uart5();
    /// TIM6 global and DAC overrun interrupts
    pub fn tim6_dac();
    /// TIM7 global interrupt
    pub fn tim7();
    /// DMA2 Channel1 global interrupt
    pub fn dma2_channel1();
    /// DMA2 Channel2 global interrupt
    pub fn dma2_channel2();
    /// DMA2 Channel3 global interrupt
    pub fn dma2_channel3();
    /// DMA2 Channel4 and DMA2 Channel5 global interrupts
    pub fn dma2_channel4_5();
    /// DMA2 Channel5 global interrupts
    pub fn dma2_channel5();
}

/// Interrupt "vector"
#[link_section=".interrupt_vector"]
pub static VECTOR: [Option<unsafe extern fn()>; 61] = [
    // OFFSET    HANDLER
    /* 0x0040 */ Some(wwdg),
    /* 0x0044 */ Some(pvd),
    /* 0x0048 */ Some(tamper_stamp),
    /* 0x004C */ Some(rtc_wkup),
    /* 0x0050 */ Some(flash),
    /* 0x0054 */ Some(rcc),
    /* 0x0058 */ Some(exti0),
    /* 0x005C */ Some(exti1),
    /* 0x0060 */ Some(exti2),
    /* 0x0064 */ Some(exti3),
    /* 0x0068 */ Some(exti4),
    /* 0x006C */ Some(dma1_channel1),
    /* 0x0070 */ Some(dma1_channel2),
    /* 0x0074 */ Some(dma1_channel3),
    /* 0x0078 */ Some(dma1_channel4),
    /* 0x007C */ Some(dma1_channel5),
    /* 0x0080 */ Some(dma1_channel6),
    /* 0x0084 */ Some(dma1_channel7),
    /* 0x0088 */ Some(adc1),
    /* 0x008C */ None,
    /* 0x0090 */ None,
    /* 0x0094 */ None,
    /* 0x0098 */ None,
    /* 0x009C */ Some(exti9_5),
    /* 0x00A0 */ Some(tim1_brk_tim15),
    /* 0x00A4 */ Some(tim1_up_tim16),
    /* 0x00A8 */ Some(tim1_trg_com_tim17),
    /* 0x00AC */ Some(tim1_cc),
    /* 0x00B0 */ Some(tim2),
    /* 0x00B4 */ Some(tim3),
    /* 0x00B8 */ Some(tim4),
    /* 0x00BC */ Some(i2c1_ev),
    /* 0x00C0 */ Some(i2c1_er),
    /* 0x00C4 */ Some(i2c2_ev),
    /* 0x00C8 */ Some(i2c2_er),
    /* 0x00CC */ Some(spi1),
    /* 0x00D0 */ Some(spi2),
    /* 0x00D4 */ Some(usart1),
    /* 0x00D8 */ Some(usart2),
    /* 0x00DC */ Some(usart3),
    /* 0x00E0 */ Some(exti15_10),
    /* 0x00E4 */ Some(rtc_alarm),
    /* 0x00E8 */ Some(cec),
    /* 0x00EC */ Some(tim12),
    /* 0x00F0 */ Some(tim13),
    /* 0x00F4 */ Some(tim14),
    /* 0x00F8 */ None,
    /* 0x00FC */ None,
    /* 0x0100 */ Some(fsmc),
    /* 0x0104 */ None,
    /* 0x0108 */ Some(tim5),
    /* 0x010C */ Some(spi3),
    /* 0x0110 */ Some(uart4),
    /* 0x0114 */ Some(uart5),
    /* 0x0118 */ Some(tim6_dac),
    /* 0x011C */ Some(tim7),
    /* 0x0120 */ Some(dma2_channel1),
    /* 0x0124 */ Some(dma2_channel2),
    /* 0x0128 */ Some(dma2_channel3),
    /* 0x012C */ Some(dma2_channel4_5),
    /* 0x0130 */ Some(dma2_channel5),
];

extern {
    #[link_name = "__GPIOA__"] static GPIOA: ::gpio::Gpio;
    #[link_name = "__GPIOB__"] static GPIOB: ::gpio::Gpio;
    #[link_name = "__GPIOC__"] static GPIOC: ::gpio::Gpio;
    #[link_name = "__RCC__"] static RCC: ::rcc::Rcc;
    #[link_name = "__TIM7__"] static TIM7: ::tim::Tim;
}

pub fn gpioa() -> &'static ::gpio::Gpio {
    &GPIOA
}

pub fn gpiob() -> &'static ::gpio::Gpio {
    &GPIOB
}

pub fn gpioc() -> &'static ::gpio::Gpio {
    &GPIOC
}

pub fn rcc() -> &'static ::rcc::Rcc {
    &RCC
}

pub fn tim7() -> &'static ::tim::Tim {
    &TIM7
}

extern {
    static __GPIOA__: ::gpio::Gpio;
    static __GPIOB__: ::gpio::Gpio;
    static __GPIOC__: ::gpio::Gpio;
    static __RCC__: ::rcc::Rcc;
    static __TIM7__: ::tim::Tim;
}

pub fn gpioa() -> &'static ::gpio::Gpio {
    &__GPIOA__
}

pub fn gpiob() -> &'static ::gpio::Gpio {
    &__GPIOB__
}

pub fn gpioc() -> &'static ::gpio::Gpio {
    &__GPIOC__
}

pub fn rcc() -> &'static ::rcc::Rcc {
    &__RCC__
}

pub fn tim7() -> &'static ::tim::Tim {
    &__TIM7__
}

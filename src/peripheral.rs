extern {
    static __AFIO__: ::afio::Afio;
    static __GPIOA__: ::gpio::Gpio;
    static __GPIOB__: ::gpio::Gpio;
    static __GPIOC__: ::gpio::Gpio;
    static __RCC__: ::rcc::Rcc;
    static __TIM7__: ::tim::Tim;
    static __USART1__: ::usart::Usart;
}

pub fn afio() -> &'static ::afio::Afio {
    &__AFIO__
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

pub fn usart1() -> &'static ::usart::Usart {
    &__USART1__
}

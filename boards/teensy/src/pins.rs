use capsules::led::ActivationMode;
use mk66;

type PinHandle = &'static mk66::gpio::Gpio<'static>;

pub unsafe fn configure_all_pins() -> (&'static [PinHandle],
                                       &'static [(PinHandle, ActivationMode)]) {
    use mk66::gpio::functions::*;
    use mk66::gpio::*;

    // The index of each pin in this array corresponds to Teensy 3.6 pinout.
    // In other words, gpio_pins[13] is Teensy pin 13, and so on.
    let gpio_pins = static_init!(
        [PinHandle; 58],
        [PB16.claim_as_gpio(), PB17.claim_as_gpio(), PD00.claim_as_gpio(),
         PA12.claim_as_gpio(), PA13.claim_as_gpio(), PD07.claim_as_gpio(),
         PD04.claim_as_gpio(), PD02.claim_as_gpio(), PD03.claim_as_gpio(),
         PC03.claim_as_gpio(), PC04.claim_as_gpio(), PC06.claim_as_gpio(),
         PC07.claim_as_gpio(), PC05.claim_as_gpio(), PD01.claim_as_gpio(),
         PC00.claim_as_gpio(), PB00.claim_as_gpio(), PB01.claim_as_gpio(),
         PB03.claim_as_gpio(), PB02.claim_as_gpio(), PD05.claim_as_gpio(),
         PD06.claim_as_gpio(), PC01.claim_as_gpio(), PC02.claim_as_gpio(),
         PE26.claim_as_gpio(), PA05.claim_as_gpio(), PA14.claim_as_gpio(),
         PA15.claim_as_gpio(), PA16.claim_as_gpio(), PB18.claim_as_gpio(),
         PB19.claim_as_gpio(), PB10.claim_as_gpio(), PB11.claim_as_gpio(),
         PE24.claim_as_gpio(), PE25.claim_as_gpio(), PC08.claim_as_gpio(),
         PC09.claim_as_gpio(), PC10.claim_as_gpio(), PC11.claim_as_gpio(),
         PA17.claim_as_gpio(), PA28.claim_as_gpio(), PA29.claim_as_gpio(),
         PA26.claim_as_gpio(), PB20.claim_as_gpio(), PB22.claim_as_gpio(),
         PB23.claim_as_gpio(), PB21.claim_as_gpio(), PD08.claim_as_gpio(),
         PD09.claim_as_gpio(), PB04.claim_as_gpio(), PB05.claim_as_gpio(),
         PD14.claim_as_gpio(), PD13.claim_as_gpio(), PD12.claim_as_gpio(),
         PD15.claim_as_gpio(), PD11.claim_as_gpio(), PE10.claim_as_gpio(),
         PE11.claim_as_gpio()]);

    let led_pins = static_init!(
            [(&'static mk66::gpio::Gpio<'static>, ActivationMode); 1],
            [(gpio_pins[13], ActivationMode::ActiveHigh)]
        );

    // UART0
    PB17.release_claim();
    PB16.release_claim();
    PB17.claim_as(UART0_TX);
    PB16.claim_as(UART0_RX);

    // SPI0
    PC04.release_claim();
    PC06.release_claim();
    PC07.release_claim();
    PA15.release_claim();
    PC06.claim_as(SPI0_MOSI);
    PC07.claim_as(SPI0_MISO);
    PA15.claim_as(SPI0_SCK);
    PC04.claim_as(SPI0_CS0);

    // SPI1
    PD05.release_claim();
    PD06.release_claim();
    PD05.claim_as(SPI1_SCK);
    PD06.claim_as(SPI1_MOSI);

    PB03.release_claim();
    PB02.release_claim();
    PB03.claim_as(I2C0_SDA0);
    PB02.claim_as(I2C0_SCLK0);

    PC11.release_claim();
    PC10.release_claim();
    PC11.claim_as(I2C1_SDA0);
    PC10.claim_as(I2C1_SCLK0);

    PA13.release_claim();
    PA14.release_claim();
    PA13.claim_as(I2C2_SDA0);
    PA14.claim_as(I2C2_SCLK0);

    // I2C3 appears not to be used at all.
    (gpio_pins, led_pins)
}


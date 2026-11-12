/// This macro allows to obtain peripherals from the one listed in the `peripherals` module
/// exported by this crate.
///
/// It makes sense to use this macro multiple times, coupled with conditional compilation (using
/// the [`cfg`
/// attribute](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)),
/// to define different setups for different boards.
///
/// # Examples
///
/// Basic usage for defining a peripheral struct holding two LED pins:
///
/// ```ignore
/// ariel_os::hal::define_peripherals!(LedPeripherals {
///     led0: P0_13,
///     led1: P0_14,
/// });
///
/// #[ariel_os::task(autostart, peripherals)]
/// async fn blinky(peripherals: LedPeripherals) {
///     let led = Output::new(peripherals.led0, Level::Low);
///     // ...
/// }
/// ```
///
/// Using `cfg` attributes to define the same struct with a different pin on each board:
///
/// ```ignore
/// #[cfg(context = "nrf52840dk")]
/// ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_13 });
///
/// #[cfg(context = "rpi-pico")]
/// ariel_os::hal::define_peripherals!(LedPeripherals { led: PIN_25 });
/// ```
///
/// Defining a `LedPin` type alias, so the type of the LED's peripheral can be named
/// the same way on every board:
///
/// ```ignore
/// #[cfg(context = "nrf52840dk")]
/// ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_13 = LedPin });
///
/// #[cfg(context = "rpi-pico")]
/// ariel_os::hal::define_peripherals!(LedPeripherals { led: PIN_25 = LedPin });
/// ```
///
// Inspired by https://github.com/adamgreig/assign-resources/tree/94ad10e2729afdf0fd5a77cd12e68409a982f58a
// under MIT license
#[macro_export]
macro_rules! define_peripherals {
    (
        $(#[$outer:meta])*
        $peripherals:ident {
            $(
                $(#[$inner:meta])*
                $peripheral_name:ident : $peripheral_field:ident $(=$peripheral_alias:ident)?
            ),*
            $(,)?
        }
    ) => {
        #[allow(dead_code,non_snake_case)]
        $(#[$outer])*
        pub struct $peripherals {
            $(
                $(#[$inner])*
                pub $peripheral_name: $crate::__peripheral_ty!($peripheral_field),
            )*
        }

        $($(
            #[allow(missing_docs, non_camel_case_types)]
            pub type $peripheral_alias = $crate::__peripheral_alias_ty!($peripheral_field);
        )?)*

        impl $crate::hal::TakePeripherals<$peripherals> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $peripherals {
                $peripherals {
                    $(
                        $(#[$inner])*
                        $peripheral_name: self.$peripheral_field.take().unwrap()
                    ),*
                }
            }
        }
    }
}

// This helper macro creates a peripheral type from its name.
// We need two variants: one for embassy hal `Peri`, one for the esp-hal style peripheral
// singletons.
// This is split out of `define_peripherals` so the gating on `esp` is done at definition time in
// this crate, and not at usage time, as that would make all crates using `define_peripherals` need
// to add a check-cfg for `esp`.
// These macros are not importable from applications as they are not part of the re-exported
// `ariel_os_hal::api`.
#[cfg(not(context = "esp"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __peripheral_ty {
    ($field:ident) => {
        $crate::hal::peripheral::Peri<'static, $crate::hal::peripherals::$field>
    };
}

#[cfg(context = "esp")]
#[macro_export]
#[doc(hidden)]
macro_rules! __peripheral_ty {
    ($field:ident) => {
        $crate::hal::peripherals::$field<'static>
    };
}

// This helper macro creates the type a peripheral alias refers to. The alias names the peripheral
// itself and not the embassy hal `Peri` wrapper. For `esp-hal` however, it does alias the
// peripheral with its lifetime parameter, as there is no distinction between a peripheral and the
// exclusive access to it, making it identical to `__peripheral_ty!()`.
#[cfg(not(context = "esp"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __peripheral_alias_ty {
    ($field:ident) => {
        $crate::hal::peripherals::$field
    };
}

#[cfg(context = "esp")]
#[macro_export]
#[doc(hidden)]
macro_rules! __peripheral_alias_ty {
    ($field:ident) => {
        $crate::hal::peripherals::$field<'static>
    };
}

/// This macro allows to group peripheral structs defined with
/// [`define_peripherals!`](crate::define_peripherals!) into a single peripheral struct.
#[macro_export]
macro_rules! group_peripherals {
    (
        $(#[$outer:meta])*
        $group:ident {
            $(
                $(#[$inner:meta])*
                $peripheral_name:ident : $peripherals:path
            ),*
            $(,)?
        }
    ) => {
        #[allow(dead_code,non_snake_case)]
        $(#[$outer])*
        pub struct $group {
            $(
                $(#[$inner])*
                pub $peripheral_name: $peripherals
            ),*
        }

        impl $crate::hal::TakePeripherals<$group> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $group {
                $group {
                    $(
                        $(#[$inner])*
                        $peripheral_name: self.take_peripherals()
                    ),*
                }
            }
        }
    }
}

/// This macro defines types which can be used as peripherals in autostart tasks for accessing
/// UARTs as defined in board descriptions.
///
/// Its argument is a comma-separated list of items that mostly go into repeated [`define_uart!`](crate::define_uart!) calls
/// (which define a struct type); see there.
///
/// In addition to that, type aliases are created for easier access; currently, that is only
/// `HOST_FACING_UART` (an alias to the type of the host facing port, on boards with
/// `has_host_facing_uart`).
///
/// While usable for applications just as well (to set up custom UARTs), it is most useful in board
/// descriptions, because in addition to the individual exposed UARTs, it can also provide extra
/// functionalities that take all UARTs into account.
#[cfg(feature = "uart")]
#[macro_export]
macro_rules! define_uarts {
    ( $( { $($args:tt)+ } ),* $(,)? ) => {
        $(
            $crate::define_uart!{ $($args)+ }
        )*

        $crate::_define_host_facing_uarts!{
            // Note that this does *not* create an outer separator; instead, the inner macro
            // creates a trailing comma for each item.
            $(
                $crate::_uart_get_host_facing_names!{ $($args)+ }
            ),*
        }
    }
}

#[cfg(not(feature = "uart"))]
#[macro_export]
macro_rules! define_uarts {
    ( $($_:tt)+ ) => {};
}

/// Creates a struct of the given `name`, containing all peripherals needed to set up a UART.
///
/// It holds the TX and RX pins, and can be taken through the same mechanism as those defined using
/// [`define_peripherals!`](crate::define_peripherals!) (that is, by using [`ariel_os::task(autostart,
/// peripherals)`](ariel_os::task()) or the underlying `TakePeripherals` trait).
///
/// The struct also has a method `.build_with_config()` that can be used to initialize an Ariel OS uart
/// instance.
#[cfg(feature = "uart")]
#[macro_export]
macro_rules! define_uart {
    ( name: $name:ident, device: $device:ident, tx: $tx:ident, rx: $rx:ident, host_facing: $_host_facing:literal ) => {
        // Rather than define_peripherals!'ing here, we define our type manually, because we also
        // have to carry the device type, and that's not coming through TakePeripherals.

        #[allow(nonstandard_style)]
        pub struct $name {
            tx: $crate::__peripheral_ty!($tx),
            rx: $crate::__peripheral_ty!($rx),
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                $name {
                    tx: self.$tx.take().unwrap(),
                    rx: self.$rx.take().unwrap(),
                }
            }
        }

        impl<'d> $name {
            pub fn build_with_config(
                self,
                rx_buf: &'d mut [u8],
                tx_buf: &'d mut [u8],
                config: $crate::hal::uart::Config,
            ) -> Result<$crate::hal::uart::Uart<'d>, $crate::uart::ConfigError> {
                $crate::hal::uart::$device::<'d>::new(self.rx, self.tx, rx_buf, tx_buf, config)
            }
        }
    };
}

#[cfg(not(feature = "uart"))]
#[macro_export]
macro_rules! define_uart {
    ( $($_:tt)+ ) => {};
}

#[macro_export]
macro_rules! _uart_get_host_facing_names {
    ( name: $name:ident, device: $_device:ident, tx: $_tx:ident, rx: $_rx:ident, host_facing: true ) => {
        $name
    };
    ( name: $_name:ident, device: $_device:ident, tx: $_tx:ident, rx: $_rx:ident, host_facing: false ) => {};
}

#[macro_export]
macro_rules! _define_host_facing_uarts {
    () => {};
    ( $name:ty ) => {
        pub type HOST_FACING_UART = $name;
    };
    ( $name:ty, $name2:ty $(, $($_:ty),+)? ) => {
        pub type HOST_FACING_UART = $name;
        // FIXME: This can only be accessed by application that have some prior knowledge of
        // multiple UARTs being available, and, more importantly, has negotiated between its
        // components which one takes which. Once any future mechanism enables that negotiation,
        // these types will need to be advertised to this mechanism.
        pub type HOST_FACING_UART_2 = $name2;
    };
    ( $name:ty, $name2:ty $(, $($_:ty),+)? ) => {
        compile_error!(
            "Larger numbers of host facing UARTs can be supported by expanding the `_define_host_facing_uarts` macro of ariel-os-hal to more cases."
        );
    };
}

/// Repeatedly calls [`define_i2c_bus`] with each item of a comma separated list.
#[cfg(feature = "i2c")]
#[macro_export]
macro_rules! define_i2c_buses {
    ( $( { $($args:tt)+ } ),* $(,)? ) => {
        $(
            $crate::define_i2c_bus!{ $($args)+ }
        )*
    }
}

#[cfg(not(feature = "i2c"))]
#[macro_export]
macro_rules! define_i2c_buses {
    ( $($_:tt)+ ) => {};
}

/// Creates a struct of the given `name`, containing all the peripherals needed to set up an I2C bus.
///
/// This also aliases the generated struct.
#[macro_export]
macro_rules! define_i2c_bus {
    ( name: $name:ident, peripheral: $peripheral:ident, sda: $sda:ident, scl: $scl:ident, aliases: [$($alias:ident,)*] ) => {

        // Because the dedicated I2C peripherals are all already taken,
        // we can only package the data and clock in the struct
        // line pins.
        #[allow(nonstandard_style)]
        pub struct $name {
            sda: $crate::__peripheral_ty!($sda),
            scl: $crate::__peripheral_ty!($scl),
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                $name {
                    sda: self.$sda.take().unwrap(),
                    scl: self.$scl.take().unwrap(),
                }
            }
        }

        impl $name {
            pub fn build_with_config(self, config: $crate::hal::i2c::controller::Config) -> $crate::hal::i2c::controller::I2c {
                $crate::hal::i2c::controller::$peripheral::new(self.sda, self.scl, config)
            }
        }

        $(
            $crate::define_i2c_alias!{$name = $alias}
        )*

    };
}

/// Aliases the sbd default name "I2c{n}" with sbd defined aliases.
#[macro_export]
macro_rules! define_i2c_alias {
    ($name:ident = $alias:ident) => {
        #[allow(nonstandard_style)]
        pub type $alias = $name;
    };
}

/// Repeatedly calls [`define_switch`] with each item of a comma separated list.
#[macro_export]
macro_rules! define_switches {
    ( $( { field_name: $fname:ident, type_name: $tname:ident, pins: [$(pin: $pin:ident, pull: $pull:ident, active: $active:ident,)+], aliases: [$($alias:ident,)*] } ),* $(,)? ) => {
        $(
            $crate::define_switch!{ name: $tname, pins:[$(pin: $pin, pull: $pull, active: $active,)+], aliases : [$($alias,)*] }
        )*

        $crate::define_peripherals!(SwitchPeripherals{
            $(
                $(
                   $fname : $pin,
                )+
            )*
        });
    };
}

/// Repeatedly calls [`define_button`] with each item of a comma separated list.
#[macro_export]
macro_rules! define_buttons {
    ( $( { field_name: $fname:ident, type_name: $tname:ident, pin: $pin:ident, pull: $pull:ident, active: $active:ident, aliases: [$($alias:ident,)*] } ),* $(,)? ) => {
        $(
            $crate::define_button!{ name: $tname, pin: $pin, pull: $pull, active: $active, aliases : [$($alias,)*] }
        )*

        $crate::define_peripherals!(ButtonPeripherals{ $($fname: $pin,)*} );
    };
}
/// Packages an [`Input`](crate::gpio::Input) and an activity [`Level`](crate::gpio::Level) into a button struct.
///
/// This creates a button struct named `name`. If the `"external-interrupts"` feature is set,
/// [`IntEnabledInput`](crate::gpio::Input) will be used and the generated struct will implement
///  `wait_for_X` async functions using the interrupts.
#[macro_export]
macro_rules! define_button {
    (name: $name:ident, pin : $pin:ident, pull : $pull:ident, active : $active:ident, aliases : [$($alias:ident,)*]) => {

        #[allow(nonstandard_style)]
        pub struct $name {
            inner: $crate::hal::button::Button
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                use $crate::gpio::Pull::*;
                use $crate::gpio::Level::*;
                let input_builder = $crate::gpio::Input::builder(
                    self.$pin.take().unwrap(),
                    $crate::select_pull!($pull, $active),
                );
                $name {
                    inner: $crate::hal::button::Button::new(
                        $crate::hal::switch::SwitchPin::new(
                            $crate::build_input!(input_builder),
                            $active,
                        )
                    )
                }
            }
        }

        impl $name {
            $crate::impl_button_inner!(inner);
        }

        $(
            $crate::define_alias!{$name = $alias}
        )*

    };
}
/// Creates a struct of the given `name`, containing all of the [`Input`](crate::gpio::Input)
/// that are part of the switch.
///
/// if the `"external-interrupts"` feature is enabled then the inputs will be [`IntEnabledInput`](crate::gpio::IntEnabledInput)
/// instead. The individual [`SwitchPin`](crate::hal::switch::SwitchPin)s, and the struct `name` itself in case of
/// single pin switched, will implement `wait_for_X` async methods using the interrupts.
#[macro_export]
macro_rules! define_switch {
    (name: $name:ident, pins: [pin : $pin:ident, pull : $pull:ident, active : $active:ident,], aliases : [$($alias:ident,)*]) => {

        #[allow(nonstandard_style)]
        pub struct $name{
            inner: $crate::hal::switch::SwitchPin
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                use $crate::gpio::Pull::*;
                use $crate::gpio::Level::*;

                let input_builder = $crate::gpio::Input::builder(
                    self.$pin.take().unwrap(),
                    $crate::select_pull!($pull, $active),
                );

                $name{
                    inner: $crate::hal::switch::SwitchPin::new(
                        $crate::build_input!(input_builder),
                        $active
                    )
                }
            }
        }

        impl $name {
            $crate::impl_switch_inner!(inner);

            pub fn pins(&mut self) -> (&mut $crate::hal::switch::SwitchPin,)
            {
                (&mut self.inner,)
            }
        }

        $(
            $crate::define_alias!{$name = $alias}
        )*

    };
    (name: $name:ident, pins: [$(pin: $pin:ident, pull: $pull:ident, active: $active:ident,)+], aliases: [$($alias:ident,)*]) => {

        #[allow(nonstandard_style)]
        pub struct $name {
            $(
                $pin: $crate::hal::switch::SwitchPin,
            )+
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                use $crate::gpio::Pull::*;
                use $crate::gpio::Level::*;

                $ (
                    let input_builder_$pin = $crate::gpio::Input::builder(
                        self.$pin.take().unwrap(),
                        $crate::select_pull!($pull, $active),
                    );
                )+

                $name {
                    $(
                        $pin: $crate::hal::switch::SwitchPin::new(
                            inner: $crate::hal::switch::SwitchPin::new(
                                $crate::build_input!(input_builder_$pin),
                                $active,
                            )
                        ),
                    )+
                }
            }
        }

        impl $name {
            pub fn pins(&mut self) -> (
                $(
                    &mut $crate::repeat!($pin, $crate::hal::switch::SwitchPin),
                )+
            )
            {
                (
                    $(
                        &mut self.$pin,
                    )+
                )
            }
        }

        $(
            $crate::define_alias!{$name = $alias}
        )*

    };
}

/// Repeatedly calls [`define_led`] with each item of a comma separated list.
#[macro_export]
macro_rules! define_leds {
    ( $( { field_name: $fname:ident, type_name: $tname:ident, pin: $pin:ident, color: $color:ident, active: $active:ident, aliases: [$($alias:ident,)*] } ),* $(,)? ) => {
        $(
            $crate::define_led!{ name: $tname, pin: $pin, color: $color, active: $active, aliases: [$($alias,)*] }
        )*

        $crate::define_peripherals!(
            LedPeripherals { $( $fname: $pin, )* }
        );

    }
}

#[macro_export]
macro_rules! define_led {
    (name: $name:ident, pin: $pin:ident, color: $color:ident, active: $active:ident, aliases: [$($alias:ident,)*]) => {

        #[allow(nonstandard_style)]
        pub struct $name {
            $color: $crate::hal::led::Led,
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                use $crate::gpio::Level::*;
                $name {
                    $color: $crate::hal::led::Led::new(
                        $crate::gpio::Output::new(
                            self.$pin.take().unwrap(),
                            !$active
                        ),
                        $active,
                    )
                }
            }
        }


        impl $name {
            pub fn turn_on(&mut self) {
                self.$color.turn_on()
            }

            pub fn turn_off(&mut self) {
                self.$color.turn_off()
            }

            pub fn toggle(&mut self) {
                self.$color.toggle()
            }

            pub fn into_output(self) -> $crate::gpio::Output<'static> {
                self.$color.into_output()
            }
        }

        $(
            $crate::define_alias!{$name = $alias}
        )*

    };
}

#[macro_export]
macro_rules! define_alias {
    ($name:ident = $alias:ident) => {
        #[allow(nonstandard_style)]
        pub type $alias = $name;
    };
    ($name:ident<$lt:lifetime> = $alias:ident) => {
        #[allow(nonstandard_style)]
        pub type $alias<$lt> = $name<$lt>;
    };
}

#[cfg(not(feature = "external-interrupts"))]
#[macro_export]
macro_rules! build_input {
    ( $input:ident ) => {
        $input.build()
    };
}
#[cfg(feature = "external-interrupts")]
#[macro_export]
macro_rules! build_input {
    ( $input:ident ) => {
        $input.build_with_interrupt().unwrap()
    };
}

#[macro_export]
macro_rules! select_pull {
    (None, High) => {
        Down
    };
    (None, Low) => {
        Up
    };
    (Down, $tt:tt) => {
        None
    };
    (Up, $tt:tt) => {
        None
    };
}

#[macro_export]
macro_rules! repeat {
    ($i: ident, $expr:ty) => {
        $expr
    };
}

#[doc(hidden)]
pub trait TakePeripherals<T> {
    fn take_peripherals(&mut self) -> T;
}

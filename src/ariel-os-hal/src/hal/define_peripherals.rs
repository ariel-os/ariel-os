/// This macro allows to obtain peripherals from the one listed in the `peripherals` module
/// exported by this crate.
///
/// It makes sense to use this macro multiple times, coupled with conditional compilation (using
/// the [`cfg`
/// attribute](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)),
/// to define different setups for different boards.
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
/// The struct also has a method `.with_config()` that can be used to initialize an Ariel OS uart
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
            pub fn with_config(
                self,
                rx_buf: &'d mut [u8],
                tx_buf: &'d mut [u8],
                config: $crate::hal::uart::Config,
            ) -> Result<$crate::hal::uart::Uart<'d>, ariel_os_embassy_common::uart::ConfigError>
            {
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

#[doc(hidden)]
pub trait TakePeripherals<T> {
    fn take_peripherals(&mut self) -> T;
}

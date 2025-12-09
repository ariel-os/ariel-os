/// This macro allows to obtain peripherals from the one listed in the `peripherals` module
/// exported by this crate.
///
/// It makes sense to use this macro multiple times, coupled with conditional compilation (using
/// the [`cfg`
/// attribute](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)),
/// to define different setups for different boards.
///
/// # Note
///
/// The [`define_peripherals!`](crate::define_peripherals!) macro expects the
/// `ariel_os::hal::peripherals` module to be in scope.
///
// Inspired by https://github.com/adamgreig/assign-resources/tree/94ad10e2729afdf0fd5a77cd12e68409a982f58a
// under MIT license
#[cfg(not(context = "esp"))]
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
                pub $peripheral_name: $crate::hal::peripheral::Peri<'static, peripherals::$peripheral_field>
            ),*
        }

        $($(
            #[allow(missing_docs, non_camel_case_types)]
            pub type $peripheral_alias = peripherals::$peripheral_field;
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

// This is almost identical to the version above, only the line that contains `Peri` is different.
// TODO: unify
#[cfg(context = "esp")]
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
                pub $peripheral_name: peripherals::$peripheral_field<'static>
            ),*
        }

        $($(
            #[allow(missing_docs, non_camel_case_types)]
            pub type $peripheral_alias = peripherals::$peripheral_field;
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

/// This macro defines a `uarts` module that gives access to UARTs by various accessors (currently:
/// through accessor types, which can be used as peripherals in autostart tasks).
///
/// Its argument is a comma-separated list of items that mostly go into repeated [`define_uart!`] calls
/// (which define a struct type); see there.
///
/// In addition to that, type aliases are created for easier access; currently, that is only
/// `HOST_FACING_UART` (an alias to the type of the host facing port, on boards with
/// `has_host_facing_uart`).
///
/// While usable for applications just as well (to set up custom UARTs), it is most useful in board
/// descriptions, because in addition to the individual exposed UARTs, it can also provide extra
/// functionalities that take all UARTs into account.
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

/// Creates a struct of the given `name` that holds the TX and RX pins, and can be taken through
/// the same mechanism as those defined using [`ariel_os::hal::define_peripherals!`] (that is, by
/// using [`ariel_os::task(autostart, peripherals)`] or the underlying `TakePeripherals` trait).
///
/// The struct also implement [`ariel_os::uart::Assignment`], and by that carries a type with the
/// (not trait-backed) promise that it is something that can be initialized through the
/// [`ariel_os::uart`] APIs.
#[macro_export]
macro_rules! define_uart {
    ( name: $name:ident, device: $device:ident, tx: $tx:ident, rx: $rx:ident, host_facing: $_host_facing:literal ) => {
        // Rather than define_peripherals!'ing here, we define our type manually, because we also
        // have to carry the device type, and that's not coming through TakePeripherals.

        #[allow(non_snake_case)]
        pub struct $name {
            tx: $crate::hal::peripheral::Peri<'static, peripherals::$tx>,
            rx: $crate::hal::peripheral::Peri<'static, peripherals::$rx>,
        }

        impl $crate::hal::TakePeripherals<$name> for &mut $crate::hal::OptionalPeripherals {
            fn take_peripherals(&mut self) -> $name {
                $name {
                    tx: self.$tx.take().unwrap(),
                    rx: self.$rx.take().unwrap(),
                }
            }
        }

        impl $crate::uart::Assignment for $name {
            type Device<'a> = $crate::hal::uart::$device<'a>;
            type Tx = $crate::hal::peripheral::Peri<'static, peripherals::$tx>;
            type Rx = $crate::hal::peripheral::Peri<'static, peripherals::$rx>;

            fn into_pins(self) -> (Self::Tx, Self::Rx) {
                (self.tx, self.rx)
            }
        }
    };
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

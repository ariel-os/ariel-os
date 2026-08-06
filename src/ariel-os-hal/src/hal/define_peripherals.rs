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

/// Repeatedly calls [``define_i2c_bus``] with each item of a comma separated list.
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

/// Packages given pins and peripherals into a struct implementing the [``crate::i2c::BusPart``] trait.
///
/// This also aliases the generated struct.
#[macro_export]
macro_rules! define_i2c_bus {
    // No aliases version
    ( name: $name:ident, peripheral: $peripheral:ident, sda: $sda:ident, scl: $scl:ident ) => {

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

        // The actual I2C peripheral (through its associated bus creating type)
        // is given through this BusPart (name tbd) trait.
        impl $crate::i2c::BusPart for $name {
            type I2CPeri = $crate::hal::i2c::controller::$peripheral;
            type Sda = $crate::__peripheral_ty!($sda);
            type Scl = $crate::__peripheral_ty!($scl);

            fn into_pins(self) -> (Self::Sda, Self::Scl) {
                (self.sda, self.scl)
            }
        }
    };
    // With aliases version
    ( name: $name:ident, peripheral: $peripheral:ident, sda: $sda:ident, scl: $scl:ident, aliases: [$($arg:ident,)+] ) => {

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

        // The actual I2C peripheral (through its associated bus creating type)
        // is given through this BusPart (name tbd) trait.
        impl $crate::i2c::BusPart for $name {
            type I2CPeri = $crate::hal::i2c::controller::$peripheral;
            type Sda = $crate::__peripheral_ty!($sda);
            type Scl = $crate::__peripheral_ty!($scl);

            fn into_pins(self) -> (Self::Sda, Self::Scl) {
                (self.sda, self.scl)
            }
        }

        $(
            $crate::define_i2c_alias!{$name = $arg}
        )*

    };
}

/// Aliases the sbd default name "i2c{n}" with sbd defined aliases.
#[macro_export]
macro_rules! define_i2c_alias {
    ($name:ident = $arg:ident) => {
        #[allow(nonstandard_style)]
        pub type $arg = $name;
    };
}

#[doc(hidden)]
pub trait TakePeripherals<T> {
    fn take_peripherals(&mut self) -> T;
}

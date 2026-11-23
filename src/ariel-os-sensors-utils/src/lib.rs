//! Provides utils useful for sensor drivers implementations.

#![no_std]
#![deny(missing_docs)]

mod atomic_state;

pub use atomic_state::AtomicState;

/// Defines an 8-bit sensor register to use in a sensor driver.
///
/// # Register Definition
///
/// This macro allows defining the register address, the individual bits of the register, as well as
/// groups of bits represented as enums.
/// It is intended to help generate the required values for control and configuration registers, not
/// to parse register values obtained from the sensor device.
///
/// This macro expects an enum named `Register` to be in scope, which must have a variant with the
/// same name as the register; the variant discriminant defines the register address, and must fit
/// into a `u8`.
/// Enums used in the macro must be in scope.
/// Bit 0 refers to the least significant bit of the register.
///
/// # Usage
///
/// The macro defines a type with associated constants with values of the whole register
/// corresponding to each individual bit.
/// The register address is exposed as the associated constant `ADDR`.
/// The type implements [`From`] to allow conversion from and to `u8`.
/// It also implements [`BitOr`](core::ops::BitOr) between both itself and the enums (automatically
/// taking into account the enum offsets inside the register), allowing to easily compose bits and
/// enum values into a register value, which can then be converted into a `u8`.
///
/// # Examples
///
/// ```
/// # use ariel_os_sensors_utils::u8_register;
/// #[derive(Copy, Clone, PartialEq, Eq)]
/// enum Register {
///     CfgRegAM = 0x60,
/// }
///
/// #[derive(Copy, Clone, PartialEq, Eq)]
/// enum MagMode {
///     Continuous,
///     Single,
///     Idle,
/// }
///
/// u8_register!(CfgRegAM {
///     [MagMode] << 0, // Defines the `MagMode` as being at bit 0.
///     COMP_TEMP_EN << 7,
///     SOFT_RST << 5,
///     LP << 4,
///     ODR << 2, // Defines the `ODR` bit as being bit 2.
/// });
///
/// // Usage
/// let addr: u8 = CfgRegAM::ADDR; // Register address.
/// let reg = CfgRegAM::COMP_TEMP_EN | MagMode::Single;
/// let reg: u8 = reg.into(); // Register value composed.
/// ```
// NOTE: known issue: the macro does not error out when the comma between an enum and a bit is
// missing.
#[macro_export]
macro_rules! u8_register {
    (
        $register:ident {
            $(
                $([$enum:path] << $enum_offset:literal)?
                $($bit:ident << $bit_offset:literal)?
                ,
            )*
        }
    ) => {
        #[derive(Debug, Copy, Clone)]
        struct $register(u8);

        impl $register {
            const ADDR: u8 = Register::$register as u8;

            $(
                $(const $bit: Self = Self(1 << $bit_offset);)?
            )*
        }

        impl core::ops::BitOr for $register {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl From<$register> for u8 {
            fn from(reg: $register) -> Self {
                reg.0
            }
        }

        impl From<u8> for $register {
            fn from(reg: u8) -> Self {
                Self(reg)
            }
        }

        $(
            $(
                impl core::ops::BitOr<$register> for $enum {
                    type Output = $register;

                    fn bitor(self, rhs: $register) -> Self::Output {
                        $register(((self as u8)  << $enum_offset) | rhs.0)
                    }
                }

                impl core::ops::BitOr<$enum> for $register {
                    type Output = Self;

                    fn bitor(self, rhs: $enum) -> Self::Output {
                        self | Self::from(rhs)
                    }
                }

                impl From<$enum> for $register {
                    fn from(value: $enum) -> Self {
                        Self((value as u8) << $enum_offset)
                    }
                }
            )?
        )*
    }
}

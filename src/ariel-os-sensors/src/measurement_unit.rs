/// Represents a unit of measurement.
///
/// # For sensor driver implementors
///
/// Missing variants can be added when required.
/// Please open an issue to discuss it.
// Built upon https://doc.riot-os.org/phydat_8h_source.html
// and https://bthome.io/format/#sensor-data
// and https://www.iana.org/assignments/senml/senml.xhtml
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MeasurementUnit {
    /// [Acceleration *g*](https://en.wikipedia.org/wiki/G-force#Unit_and_measurement).
    AccelG,
    /// Ampere (A).
    Ampere,
    /// Becquerel (Bq).
    Becquerel,
    /// Logic boolean: `0` means `false` and `1` means `true`.
    Bool,
    /// Candela (cd).
    Candela,
    /// Degrees Celsius (°C).
    Celsius,
    /// Coulomb (C).
    Coulomb,
    /// Decibel (dB).
    Decibel,
    /// Decimal degrees (°).
    DecimalDegree,
    /// Degrees (°).
    Degree,
    /// Farad (F).
    Farad,
    // FIXME: Kilogram as well?
    /// Gram (g).
    Gram,
    /// Gray (Gy).
    Gray,
    /// Henry (H).
    Henry,
    /// Hertz (Hz).
    Hertz,
    /// Joule (J).
    Joule,
    /// Katal (kat).
    Katal,
    /// Kelvin (K).
    Kelvin,
    /// Lumen (lm).
    Lumen,
    /// Lux (lx).
    Lux,
    /// Meter (m)
    Meter,
    /// Meter per second (m/s).
    MeterPerSecond,
    /// Mole (mol).
    Mole,
    /// Newton (N).
    Newton,
    /// Ohm (Ω).
    Ohm,
    /// Pascal (Pa).
    Pascal,
    /// Percent (%).
    Percent,
    /// %RH.
    PercentageRelativeHumidity,
    /// Radian (rad).
    Radian,
    /// Second (s).
    Second,
    /// Siemens (S).
    Siemens,
    /// Sievert (Sv).
    Sievert,
    /// Steradian (sr).
    Steradian,
    /// Tesla (T).
    Tesla,
    /// Volt (V).
    Volt,
    /// Watt (W).
    Watt,
    /// Weber (Wb).
    Weber,
}

macro_rules! provide_unit_fmt {
    ($unit:expr, $f:expr) => {
        match $unit {
            Self::AccelG => write!($f, "g"),
            Self::Ampere => write!($f, "A"),
            Self::Becquerel => write!($f, "Bq"),
            Self::Bool => write!($f, ""),
            Self::Candela => write!($f, "cd"),
            // As recommended by the Unicode Standard v16 (U+00B0 + U+0043)
            Self::Celsius => write!($f, "°C"),
            Self::Coulomb => write!($f, "C"),
            Self::Decibel => write!($f, "dB"),
            Self::DecimalDegree => write!($f, "°"),
            Self::Degree => write!($f, "°"),
            Self::Farad => write!($f, "F"),
            Self::Gram => write!($f, "g"),
            Self::Gray => write!($f, "Gy"),
            Self::Henry => write!($f, "H"),
            Self::Hertz => write!($f, "Hz"),
            Self::Joule => write!($f, "J"),
            Self::Katal => write!($f, "kat"),
            Self::Kelvin => write!($f, "K"),
            Self::Lumen => write!($f, "lm"),
            Self::Lux => write!($f, "lx"),
            Self::Meter => write!($f, "m"),
            Self::MeterPerSecond => write!($f, "m/s"),
            Self::Mole => write!($f, "mol"),
            Self::Newton => write!($f, "N"),
            Self::Ohm => write!($f, "Ω"),
            Self::Pascal => write!($f, "Pa"),
            Self::Percent => write!($f, "%"),
            Self::PercentageRelativeHumidity => write!($f, "%RH"),
            Self::Radian => write!($f, "rad"),
            Self::Second => write!($f, "s"),
            Self::Siemens => write!($f, "S"),
            Self::Sievert => write!($f, "Sv"),
            Self::Steradian => write!($f, "sr"),
            Self::Tesla => write!($f, "T"),
            Self::Volt => write!($f, "V"),
            Self::Watt => write!($f, "W"),
            Self::Weber => write!($f, "Wb"),
        }
    };
}

impl core::fmt::Display for MeasurementUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        provide_unit_fmt!(self, f)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for MeasurementUnit {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;

        provide_unit_fmt!(self, f);
    }
}

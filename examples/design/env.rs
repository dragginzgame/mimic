use crate::{prelude::*, world};
use base::{types, validator::number::Range};

///
/// Area
/// unit = m²
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "types::math::DecimalFormat<6, 3>"), default = 1.0),
    traits(remove(Validate))
)]
pub struct Area {}

#[constant(ty = "i32", value = 0)]
pub struct AREA_MIN {}

#[constant(ty = "i32", value = 1_000_000)]
pub struct AREA_MAX {}

impl Validate for Area {
    fn validate(&self) -> Result<(), ErrorVec> {
        Range::validate(&self.0, AREA_MIN, AREA_MAX).map_err(ErrorVec::from)
    }
}

///
/// AreaGuide
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "world::env::Area")),
    guide(
        entry(name = "1m²", value = 1.0),
        entry(name = "2m²", value = 2.0),
        entry(name = "5m²", value = 5.0)
    )
)]
pub struct AreaGuide {}

///
/// Brightness
///

#[newtype(
    primitive = "U8",
    value(item(is = "types::U8"), default = 5),
    traits(remove(Validate))
)]
pub struct Brightness {}

#[constant(ty = "i32", value = 0)]
pub struct BRIGHTNESS_MIN {}

#[constant(ty = "i32", value = 10)]
pub struct BRIGHTNESS_MAX {}

impl Validate for Brightness {
    fn validate(&self) -> Result<(), ErrorVec> {
        Range::validate(&self.0, BRIGHTNESS_MIN, BRIGHTNESS_MAX).map_err(ErrorVec::from)
    }
}

///
/// BrightnessGuide
///

#[newtype(
    primitive = "U8",
    value(item(is = "world::env::Brightness")),
    guide(
        entry(name = "Pitch Black", value = 0),
        entry(name = "Average", value = 5),
        entry(name = "Searing Light", value = 10)
    )
)]
pub struct BrightnessGuide {}

///
/// Distance
/// unit = m
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "types::math::DecimalFormat<6, 3>"), default = 1.0),
    traits(remove(Validate))
)]
pub struct Distance {}

#[constant(ty = "i32", value = 0)]
pub struct DISTANCE_MIN {}

#[constant(ty = "i32", value = 1_000_000)]
pub struct DISTANCE_MAX {}

impl Validate for Distance {
    fn validate(&self) -> Result<(), ErrorVec> {
        Range::validate(&self.0, DISTANCE_MIN, DISTANCE_MAX).map_err(ErrorVec::from)
    }
}

///
/// DistanceGuide
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "world::env::Distance")),
    guide(
        entry(name = "50cm", value = 0.5),
        entry(name = "1m", value = 1.0),
        entry(name = "2m", value = 2.0),
        entry(name = "5m", value = 5.0)
    )
)]
pub struct DistanceGuide {}

///
/// Size
/// unit = m
///

#[newtype(
    primitive = "U32",
    value(item(is = "types::U32"), default = 1),
    traits(remove(Validate))
)]
pub struct Size {}

#[constant(ty = "i32", value = 0)]
pub struct SIZE_MIN {}

#[constant(ty = "i32", value = 10_000)]
pub struct SIZE_MAX {}

impl Validate for Size {
    fn validate(&self) -> Result<(), ErrorVec> {
        Range::validate(&self.0, SIZE_MIN, SIZE_MAX).map_err(ErrorVec::from)
    }
}

///
/// SizeGuide
///

#[newtype(
    primitive = "U32",
    value(item(is = "world::env::Size")),
    guide(
        entry(name = "1kg", value = 1),
        entry(name = "2kg", value = 2),
        entry(name = "5kg", value = 5)
    )
)]
pub struct SizeGuide {}

///
/// Temperature
/// unit = °C
///

#[newtype(
    primitive = "I16",
    value(item(is = "types::I16"), default = 20),
    guide(
        entry(name = "Coldest Possible", value = -100),
        entry(name = "Water Freezes", value = 0),
        entry(name = "Room Temperature", value = 20),
        entry(name = "Water Boils", value = 100)
    ),
    traits(remove(Validate))
)]
pub struct Temperature {}

#[constant(ty = "i32", value = -100)]
pub struct TEMPERATURE_MIN {}

#[constant(ty = "i32", value = 10_000)]
pub struct TEMPERATURE_MAX {}

impl Validate for Temperature {
    fn validate(&self) -> Result<(), ErrorVec> {
        Range::validate(&self.0, TEMPERATURE_MIN, TEMPERATURE_MAX).map_err(ErrorVec::from)
    }
}

///
/// TemperatureGuide
///

#[newtype(
    primitive = "I16",
    value(item(is = "world::env::Temperature")),
    guide(
        entry(name = "Coldest Possible", value = -100),
        entry(name = "Water Freezes", value = 0),
        entry(name = "Room Temperature", value = 20),
        entry(name = "Water Boils", value = 100)
    )
)]
pub struct TemperatureGuide {}

///
/// Volume
/// unit = m³
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "types::math::DecimalFormat<6, 3>"), default = 1.0),
    traits(remove(Validate))
)]
pub struct Volume {}

#[constant(ty = "i32", value = 0)]
pub struct VOLUME_MIN {}

#[constant(ty = "i32", value = 10_000)]
pub struct VOLUME_MAX {}

impl Validate for Volume {
    fn validate(&self) -> Result<(), ErrorVec> {
        Range::validate(&self.0, VOLUME_MIN, VOLUME_MAX).map_err(ErrorVec::from)
    }
}

///
/// VolumeGuide
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "world::env::Volume")),
    guide(
        entry(name = "1m³", value = 1.0),
        entry(name = "2m³", value = 2.0),
        entry(name = "5m³", value = 5.0)
    )
)]
pub struct VolumeGuide {}

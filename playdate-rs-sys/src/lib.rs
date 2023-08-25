#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(target_arch = "arm", target_os = "none"))]
include!("./thumbv7em_bindings.rs");

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct LCDColor(u64);

impl LCDColor {
    /// Convert the LCDColor to a LCDSolidColor
    pub fn as_solid_color(&self) -> Option<LCDSolidColor> {
        match self.0 {
            0 => Some(LCDSolidColor::kColorBlack),
            1 => Some(LCDSolidColor::kColorWhite),
            2 => Some(LCDSolidColor::kColorClear),
            3 => Some(LCDSolidColor::kColorXOR),
            _ => None,
        }
    }

    /// Convert the LCDColor to a LCDPattern
    ///
    /// # Safety
    ///
    /// This function is unsafe because it casts the LCDColor into a raw pointer and dereferences it.
    /// The caller must ensure that the pointer is valid before calling this method.
    pub unsafe fn as_pattern(&self) -> Option<LCDPattern> {
        match self.0 {
            x if x <= 3 => None,
            _ => Some(unsafe { *(self.0 as *const LCDPattern) }),
        }
    }
}

impl From<LCDSolidColor> for LCDColor {
    fn from(value: LCDSolidColor) -> Self {
        LCDColor(value as _)
    }
}

impl From<&LCDPattern> for LCDColor {
    fn from(value: &LCDPattern) -> Self {
        LCDColor(value as *const LCDPattern as _)
    }
}

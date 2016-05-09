use std;
use std::fmt;

/// Types of icon elements that can be decoded as images or masks.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IconType {
    /// 16x16 24-bit icon (without alpha).
    RGB24_16x16,
    /// 16x16 8-bit alpha mask.
    Mask8_16x16,
    /// 32x32 24-bit icon (without alpha).
    RGB24_32x32,
    /// 32x32 8-bit alpha mask.
    Mask8_32x32,
    /// 128x128 24-bit icon (without alpha).
    RGB24_128x128,
    /// 128x128 8-bit alpha mask.
    Mask8_128x128,
    /// 256x256 32-bit icon.
    RGBA32_256x256,
    /// 256x256 32-bit icon at 2x "retina" density (so, 512 by 512 pixels).
    RGBA32_256x256_2x,
    /// 512x512 32-bit icon.
    RGBA32_512x512,
    /// 512x512 32-bit icon at 2x "retina" density (so, 1024 by 1024 pixels).
    RGBA32_512x512_2x,
}

impl IconType {
    /// Get the icon type associated with the given OSType, if any.
    pub fn from_ostype(ostype: OSType) -> Option<IconType> {
        let OSType(raw_ostype) = ostype;
        match &raw_ostype {
            b"is32" => Some(IconType::RGB24_16x16),
            b"s8mk" => Some(IconType::Mask8_16x16),
            b"il32" => Some(IconType::RGB24_32x32),
            b"l8mk" => Some(IconType::Mask8_32x32),
            b"it32" => Some(IconType::RGB24_128x128),
            b"t8mk" => Some(IconType::Mask8_128x128),
            b"ic08" => Some(IconType::RGBA32_256x256),
            b"ic14" => Some(IconType::RGBA32_256x256_2x),
            b"ic09" => Some(IconType::RGBA32_512x512),
            b"ic10" => Some(IconType::RGBA32_512x512_2x),
            _ => None,
        }
    }

    /// Get the OSType that represents this icon type.
    pub fn ostype(self) -> OSType {
        match self {
            IconType::RGB24_16x16 => OSType(*b"is32"),
            IconType::Mask8_16x16 => OSType(*b"s8mk"),
            IconType::RGB24_32x32 => OSType(*b"il32"),
            IconType::Mask8_32x32 => OSType(*b"l8mk"),
            IconType::RGB24_128x128 => OSType(*b"it32"),
            IconType::Mask8_128x128 => OSType(*b"t8mk"),
            IconType::RGBA32_256x256 => OSType(*b"ic08"),
            IconType::RGBA32_256x256_2x => OSType(*b"ic14"),
            IconType::RGBA32_512x512 => OSType(*b"ic09"),
            IconType::RGBA32_512x512_2x => OSType(*b"ic10"),
        }
    }

    /// Returns the pixel data width of this icon type.  Normally this is the
    /// same as the screen width, but for 2x "retina" density icons, this will
    /// be twice that value.
    ///
    /// # Examples
    /// ```
    /// use icns::IconType;
    /// assert_eq!(IconType::Mask8_128x128.pixel_width(), 128);
    /// assert_eq!(IconType::RGBA32_256x256.pixel_width(), 256);
    /// assert_eq!(IconType::RGBA32_256x256_2x.pixel_width(), 512);
    /// ```
    pub fn pixel_width(self) -> u32 {
        match self {
            IconType::RGB24_16x16 => 16,
            IconType::Mask8_16x16 => 16,
            IconType::RGB24_32x32 => 32,
            IconType::Mask8_32x32 => 32,
            IconType::RGB24_128x128 => 128,
            IconType::Mask8_128x128 => 128,
            IconType::RGBA32_256x256 => 256,
            IconType::RGBA32_256x256_2x => 512,
            IconType::RGBA32_512x512 => 512,
            IconType::RGBA32_512x512_2x => 1024,
        }
    }

    /// Returns the screen width of this icon type.  Normally this is the same
    /// as the pixel width, but for 2x "retina" density icons, this will be
    /// half that value.
    ///
    /// # Examples
    /// ```
    /// use icns::IconType;
    /// assert_eq!(IconType::Mask8_128x128.screen_width(), 128);
    /// assert_eq!(IconType::RGBA32_256x256.screen_width(), 256);
    /// assert_eq!(IconType::RGBA32_256x256_2x.screen_width(), 256);
    /// ```
    pub fn screen_width(self) -> u32 {
        match self {
            IconType::RGB24_16x16 => 16,
            IconType::Mask8_16x16 => 16,
            IconType::RGB24_32x32 => 32,
            IconType::Mask8_32x32 => 32,
            IconType::RGB24_128x128 => 128,
            IconType::Mask8_128x128 => 128,
            IconType::RGBA32_256x256 => 256,
            IconType::RGBA32_256x256_2x => 256,
            IconType::RGBA32_512x512 => 512,
            IconType::RGBA32_512x512_2x => 512,
        }
    }
}

/// A Macintosh OSType (also known as a ResType), used in ICNS files to
/// identify the type of each icon element.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OSType(pub [u8; 4]);

impl fmt::Display for OSType {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let &OSType(raw) = self;
        for &byte in &raw {
            let character = std::char::from_u32(u32::from(byte)).unwrap();
            try!(write!(out, "{}", character));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_type_ostype_round_trip() {
        let icon_types = [IconType::RGB24_16x16,
                          IconType::Mask8_16x16,
                          IconType::RGB24_32x32,
                          IconType::Mask8_32x32,
                          IconType::RGB24_128x128,
                          IconType::Mask8_128x128,
                          IconType::RGBA32_256x256,
                          IconType::RGBA32_256x256_2x,
                          IconType::RGBA32_512x512,
                          IconType::RGBA32_512x512_2x];
        for icon_type in &icon_types {
            let ostype = icon_type.ostype();
            let from = IconType::from_ostype(ostype);
            assert_eq!(Some(*icon_type), from);
        }
    }
}

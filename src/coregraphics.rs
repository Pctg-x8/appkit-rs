//! Core Graphics

use objc::{Encode, Encoding};
use libc::*;
use {ExternalRc, ExternalRced};

/// A unique identifier for an attached display.
pub type CGDirectDisplayID = u32;
#[cfg(target_pointer_width = "64")] pub type CGFloat = f64;
#[cfg(not(target_pointer_width = "64"))] pub type CGFloat = f32;
#[repr(C)] #[derive(Debug, Clone, PartialEq)] pub struct CGPoint { pub x: CGFloat, pub y: CGFloat }
#[repr(C)] #[derive(Debug, Clone, PartialEq)] pub struct CGSize  { pub width: CGFloat, pub height: CGFloat }
#[repr(C)] #[derive(Debug, Clone, PartialEq)] pub struct CGRect  { pub origin: CGPoint, pub size: CGSize }
unsafe impl Encode for CGPoint
{
    fn encode() -> Encoding
    {
        unsafe
        {
            Encoding::from_str(&format!("{{CGPoint={}{}}}", CGFloat::encode().as_str(), CGFloat::encode().as_str()))
        }
    }
}
unsafe impl Encode for CGSize
{
    fn encode() -> Encoding
    {
        unsafe
        {
            Encoding::from_str(&format!("{{CGSize={}{}}}", CGFloat::encode().as_str(), CGFloat::encode().as_str()))
        }
    }
}
unsafe impl Encode for CGRect
{
    fn encode() -> Encoding
    {
        unsafe
        {
            Encoding::from_str(&format!("{{CGRect={}{}}}", CGPoint::encode().as_str(), CGSize::encode().as_str()))
        }
    }
}

/// A set of components that define a color, with a color space specifying how to interpret them.
pub enum CGColor {}
/// A set of components that define a color, with a color space specifying how to interpret them.
pub type CGColorRef = *mut CGColor;

/// An index into a font table.
pub type CGFontIndex = c_ushort;
/// An index into the internal glyph table of a font.
pub type CGGlyph = CGFontIndex;

/// An affine transformation matrix for use in drawing 2D graphics.
#[repr(C)] pub struct CGAffineTransform {
    pub a: CGFloat, pub b: CGFloat, pub c: CGFloat, pub d: CGFloat,
    pub tx: CGFloat, pub ty: CGFloat
}

/// A set of character glyphs and layout information for drawing text.
pub enum CGFont {}
/// A set of character glyphs and layout information for drawing text.
pub type CGFontRef = *mut CGFont;
impl ExternalRced for CGFont {
    unsafe fn own_from_unchecked(p: *mut Self) -> ExternalRc<Self> {
        ExternalRc::with_fn(p, CGFontRetain, CGFontRelease)
    }
}

#[link(name = "CoreGraphics", kind = "framework")] extern "system" {
    fn CGFontRelease(font: CGFontRef);
    fn CGFontRetain(font: CGFontRef) -> CGFontRef;
}

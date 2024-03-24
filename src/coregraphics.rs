//! Core Graphics

use crate::ExternalRc;
use appkit_rs_derive::external_refcounted;
use libc::*;
use objc::{Encode, Encoding};
use std::ptr::null;

/// A unique identifier for an attached display.
pub type CGDirectDisplayID = u32;
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = f32;
#[cfg(target_pointer_width = "64")]
pub const CGFLOAT_MAX: CGFloat = ::std::f64::MAX;
#[cfg(not(target_pointer_width = "64"))]
pub const CGFLOAT_MAX: CGFloat = ::std::f32::MAX;
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
unsafe impl Encode for CGPoint {
    fn encode() -> Encoding {
        unsafe {
            Encoding::from_str(&format!(
                "{{CGPoint={}{}}}",
                CGFloat::encode().as_str(),
                CGFloat::encode().as_str()
            ))
        }
    }
}
unsafe impl Encode for CGSize {
    fn encode() -> Encoding {
        unsafe {
            Encoding::from_str(&format!(
                "{{CGSize={}{}}}",
                CGFloat::encode().as_str(),
                CGFloat::encode().as_str()
            ))
        }
    }
}
unsafe impl Encode for CGRect {
    fn encode() -> Encoding {
        unsafe {
            Encoding::from_str(&format!(
                "{{CGRect={}{}}}",
                CGPoint::encode().as_str(),
                CGSize::encode().as_str()
            ))
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
#[repr(C)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}
/// Identity scale, no rotation and transform
impl Default for CGAffineTransform {
    fn default() -> Self {
        CGAffineTransform {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

/// A set of character glyphs and layout information for drawing text.
#[external_refcounted(CGFontRetain, CGFontRelease)]
pub enum CGFont {}
/// A set of character glyphs and layout information for drawing text.
pub type CGFontRef = *mut CGFont;

#[external_refcounted(CGPathRetain, CGPathRelease)]
pub enum CGPath {}
/// An immutable graphics path: a mathmatical description of shapes or lines to be drawn in a graphics context.
pub type CGPathRef = *mut CGPath;
/// A mutable graphics path: a mathematical description of shapes or lines to be drawn in a graphics context.
pub type CGMutablePathRef = *mut CGPath;
impl CGPath {
    /// Create an immutable path of a rectangle.
    pub fn new_rect(r: CGRect, transform: Option<&CGAffineTransform>) -> Result<ExternalRc<Self>, ()> {
        let transform_ptr = transform.map_or_else(null, |p| p as *const _);

        unsafe { ExternalRc::retained_checked(CGPathCreateWithRect(r, transform_ptr)).ok_or(()) }
    }
    /// Creates a mutable graphics path.
    pub fn new_mutable() -> Result<ExternalRc<Self>, ()> {
        unsafe { ExternalRc::retained_checked(CGPathCreateMutable()).ok_or(()) }
    }

    /// Appends a path to onto a mutable graphics path.
    pub fn add_path(&mut self, p: &Self, transform: Option<&CGAffineTransform>) {
        let ptf = transform.map_or_else(null, |p| p as _);

        unsafe { CGPathAddPath(self as _, ptf, p as *const _ as _) }
    }
    /// For each element in a graphics path, calls a custom applier function.
    pub unsafe fn apply_raw(&self, ctx: *mut c_void, fnptr: CGPathApplierFunction) {
        CGPathApply(self as *const _ as _, ctx, fnptr)
    }
    /// For each element in a graphics path, calls a custom applier function. (safety version)
    pub fn apply<F>(&self, mut callback: F)
    where
        F: FnMut(&CGPathElement),
    {
        extern "C" fn cb_wrap<F>(ctx: *mut c_void, element: *const CGPathElement)
        where
            F: FnMut(&CGPathElement),
        {
            let callback = unsafe { (ctx as *mut F).as_mut().unwrap() };
            callback(unsafe { element.as_ref().unwrap() });
        }
        unsafe {
            self.apply_raw(&mut callback as *mut F as _, cb_wrap::<F>);
        }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CGPathElement {
    pub type_: CGPathElementType,
    pub points: *mut CGPoint,
}
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CGPathElementType {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubpath,
}

pub type CGPathApplierFunction = extern "C" fn(info: *mut c_void, element: *const CGPathElement);
#[link(name = "CoreGraphics", kind = "framework")]
extern "system" {
    fn CGFontRelease(font: CGFontRef);
    fn CGFontRetain(font: CGFontRef) -> CGFontRef;
    fn CGPathCreateWithRect(rect: CGRect, transform: *const CGAffineTransform) -> CGPathRef;
    fn CGPathCreateMutable() -> CGMutablePathRef;
    fn CGPathRelease(path: CGPathRef);
    fn CGPathRetain(path: CGPathRef) -> CGPathRef;
    fn CGPathAddPath(path1: CGMutablePathRef, m: *const CGAffineTransform, path2: CGPathRef);
    fn CGPathApply(path: CGPathRef, info: *mut c_void, function: CGPathApplierFunction);
}

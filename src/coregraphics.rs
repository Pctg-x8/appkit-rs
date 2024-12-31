//! Core Graphics

use crate::{opt_pointer, CoreObject, CoreRetainedMutableObject, CoreRetainedObject};
use libc::*;
use objc::{Encode, Encoding};

/// A unique identifier for an attached display.
pub type CGDirectDisplayID = u32;

#[cfg(target_pointer_width = "64")]
pub type CGFloat = core::ffi::c_double;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = core::ffi::c_float;
pub const CGFLOAT_MAX: CGFloat = CGFloat::MAX;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
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

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
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

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
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

DefineCoreObject! {
    /// A set of components that define a color, with a color space specifying how to interpret them.
    pub CGColor;
}
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
    #[inline(always)]
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

DefineOpaqueFFIObject! {
    /// A set of character glyphs and layout information for drawing text.
    pub struct CGFont;
}
unsafe impl CoreObject for CGFont {
    #[inline(always)]
    unsafe fn retain(ptr: *const Self) {
        CGFontRetain(ptr as _);
    }

    #[inline(always)]
    unsafe fn release(ptr: *const Self) {
        CGFontRelease(ptr as _);
    }
}
/// A set of character glyphs and layout information for drawing text.
pub type CGFontRef = *mut CGFont;

DefineOpaqueFFIObject! {
    pub struct CGPath;
}
unsafe impl CoreObject for CGPath {
    #[inline(always)]
    unsafe fn retain(ptr: *const Self) {
        CGPathRetain(ptr);
    }

    #[inline(always)]
    unsafe fn release(ptr: *const Self) {
        CGPathRelease(ptr);
    }
}
/// An immutable graphics path: a mathmatical description of shapes or lines to be drawn in a graphics context.
pub type CGPathRef = *const CGPath;
/// A mutable graphics path: a mathematical description of shapes or lines to be drawn in a graphics context.
pub type CGMutablePathRef = *mut CGPath;
impl CGPath {
    /// Create an immutable path of a rectangle.
    #[inline(always)]
    pub fn new_rect(r: CGRect, transform: Option<&CGAffineTransform>) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe { CoreRetainedObject::retained(CGPathCreateWithRect(r, opt_pointer(transform))).ok_or(()) }
    }

    /// Creates a mutable graphics path.
    #[inline(always)]
    pub fn new_mutable() -> Result<CoreRetainedMutableObject<Self>, ()> {
        unsafe { CoreRetainedMutableObject::from_retained_ptr(CGPathCreateMutable()).ok_or(()) }
    }

    /// Appends a path to onto a mutable graphics path.
    #[inline(always)]
    pub fn add_path(&mut self, p: &Self, transform: Option<&CGAffineTransform>) {
        unsafe { CGPathAddPath(self, opt_pointer(transform), p) }
    }

    /// For each element in a graphics path, calls a custom applier function.
    #[inline(always)]
    pub unsafe fn apply_raw(&self, ctx: *mut c_void, fnptr: CGPathApplierFunction) {
        CGPathApply(self, ctx, fnptr)
    }

    /// For each element in a graphics path, calls a custom applier function. (safety version)
    pub fn apply<F: FnMut(&CGPathElement)>(&self, mut callback: F) {
        extern "C" fn cb_wrap<F: FnMut(&CGPathElement)>(ctx: *mut c_void, element: *const CGPathElement) {
            let callback = unsafe { (ctx as *mut F).as_mut().unwrap() };
            callback(unsafe { element.as_ref().unwrap() });
        }

        unsafe {
            self.apply_raw(&mut callback as *mut F as _, cb_wrap::<F>);
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
unsafe extern "system" {
    unsafe fn CGFontRelease(font: CGFontRef);
    unsafe fn CGFontRetain(font: CGFontRef) -> CGFontRef;
    unsafe fn CGPathCreateWithRect(rect: CGRect, transform: *const CGAffineTransform) -> CGPathRef;
    unsafe fn CGPathCreateMutable() -> CGMutablePathRef;
    unsafe fn CGPathRelease(path: CGPathRef);
    unsafe fn CGPathRetain(path: CGPathRef) -> CGPathRef;
    unsafe fn CGPathAddPath(path1: CGMutablePathRef, m: *const CGAffineTransform, path2: CGPathRef);
    unsafe fn CGPathApply(path: CGPathRef, info: *mut c_void, function: CGPathApplierFunction);
}

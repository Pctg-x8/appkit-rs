//! Core Text

use crate::{
    CFArray, CFArrayRef, CFAttributedString, CFAttributedStringRef, CFData, CFDataRef, CFDictionary, CFDictionaryRef,
    CFIndex, CFRange, CFStringRef, CGAffineTransform, CGFloat, CGFont, CGFontRef, CGGlyph, CGPath, CGPathRef, CGPoint,
    CGRect, CGSize, CoreObject, CoreRetainedObject, ExternalRc, NSFont, UniChar,
};
use core::ops::Range;
use core::ptr::{null, null_mut};
use core::slice;
use std::borrow::Cow;

/// An opaque type represents a Core Text font object.
pub enum CTFont {}
/// A reference to a Core Text font object.
pub type CTFontRef = *mut CTFont;

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CTFontSymbolicTraits {
    ItalicTrait = 1 << 0,
    BoldTrait = 1 << 1,
}
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CTFontOrientation {
    Default = 0,
    Horizontal,
    Vertical,
}

TollfreeBridge!(NSFont = CTFont);
unsafe impl CoreObject for CTFont {}
impl CTFont {
    /// Creates a new font reference from an existing Core Graphics font reference.
    pub fn from_cg(
        graphics_font: &CGFont,
        size: CGFloat,
        matrix: Option<&CGAffineTransform>,
        attributes: Option<&CTFontDescriptor>,
    ) -> Result<CoreRetainedObject<Self>, ()> {
        let matrix_ref = matrix.map_or_else(null, |p| p as _);
        let attrs_ref = attributes.map_or_else(null_mut, |p| p as *const _ as _);

        unsafe {
            CoreRetainedObject::retained_checked(CTFontCreateWithGraphicsFont(
                graphics_font as *const _ as _,
                size,
                matrix_ref,
                attrs_ref,
            ))
            .ok_or(())
        }
    }
    /// Returns a Core Graphics font reference.
    pub fn to_cg(&self) -> Result<ExternalRc<CGFont>, ()> {
        unsafe { ExternalRc::retained_checked(CTFontCopyGraphicsFont(self as *const _ as _, null_mut())).ok_or(()) }
    }
    /// Returns a Core Graphics font reference and attributes.
    pub fn to_cg_with_attributes(&self) -> Result<(ExternalRc<CGFont>, CoreRetainedObject<CTFontDescriptor>), ()> {
        let mut attrs = null_mut();
        let font_ptr = unsafe { CTFontCopyGraphicsFont(self as *const _ as _, &mut attrs) };
        let f = unsafe { ExternalRc::retained_checked(font_ptr).ok_or(())? };
        let d = unsafe { CoreRetainedObject::retained_checked(attrs).ok_or(())? };

        Ok((f, d))
    }

    /// Returns a new font reference that best matches the given font descriptor
    pub fn from_font_descriptor(
        descriptor: &CTFontDescriptor,
        size: CGFloat,
        matrix: Option<&CGAffineTransform>,
    ) -> Result<CoreRetainedObject<Self>, ()> {
        let matrix_ref = matrix.map_or_else(null, |p| p as _);

        unsafe {
            CoreRetainedObject::retained_checked(CTFontCreateWithFontDescriptor(
                descriptor as *const _ as _,
                size,
                matrix_ref,
            ))
            .ok_or(())
        }
    }

    /// Returns an array of languages supported by the font.
    pub fn supported_languages(&self) -> Result<CoreRetainedObject<CFArray>, ()> {
        unsafe { CoreRetainedObject::retained_checked(CTFontCopySupportedLanguages(self as *const _ as _)).ok_or(()) }
    }
    /// Provides basic Unicode encoding for the given font, returning by reference an array of `CGGlyph` values
    /// corresponding to a given array of Unicode characters for the given font.
    pub fn glyphs_for_characters(&self, characters: &[UniChar]) -> Result<Vec<CGGlyph>, ()> {
        let mut glyphs = Vec::with_capacity(characters.len());
        unsafe {
            glyphs.set_len(characters.len());
        }
        let r = unsafe {
            CTFontGetGlyphsForCharacters(
                self as *const _ as _,
                characters.as_ptr(),
                glyphs.as_mut_ptr(),
                characters.len() as _,
            )
        };

        if !r {
            Err(())
        } else {
            Ok(glyphs)
        }
    }
    /// Creates a path for the specified glyph.
    pub fn create_path_for_glyph(
        &self,
        glyph: CGGlyph,
        transform: Option<&CGAffineTransform>,
    ) -> Result<ExternalRc<CGPath>, ()> {
        let ptf = transform.map_or_else(null, |p| p as _);

        unsafe { ExternalRc::retained_checked(CTFontCreatePathForGlyph(self as *const _ as _, glyph, ptf)).ok_or(()) }
    }

    /// Returns a new font with additional attributes based on the original font.
    pub fn create_copy_with_attributes(
        &self,
        size: CGFloat,
        transform: Option<&CGAffineTransform>,
        attributes: Option<&CTFontDescriptor>,
    ) -> Result<CoreRetainedObject<Self>, ()> {
        let transform_ptr = transform.map_or_else(null, |p| p as _);
        let attributes_ptr = attributes.map_or_else(null, |p| p as _);

        unsafe {
            CoreRetainedObject::retained_checked(CTFontCreateCopyWithAttributes(
                self as *const _ as _,
                size,
                transform_ptr,
                attributes_ptr as *const _ as _,
            ))
            .ok_or(())
        }
    }
}

/// Font Metrics
impl CTFont {
    /// Returns the point size of the font.
    #[inline]
    pub fn size(&self) -> CGFloat {
        unsafe { CTFontGetSize(self as *const _ as _) }
    }

    /// Returns the scaled font-ascent metric of the font.
    #[inline]
    pub fn ascent(&self) -> CGFloat {
        unsafe { CTFontGetAscent(self as *const _ as _) }
    }

    /// Returns the scaled font-descent metric of the font.
    #[inline]
    pub fn descent(&self) -> CGFloat {
        unsafe { CTFontGetDescent(self as *const _ as _) }
    }

    /// Returns the cap-height metric of the font.
    #[inline]
    pub fn cap_height(&self) -> CGFloat {
        unsafe { CTFontGetCapHeight(self as *const _ as _) }
    }

    /// Returns the x-height metric of the font.
    #[inline]
    pub fn x_height(&self) -> CGFloat {
        unsafe { CTFontGetXHeight(self as *const _ as _) }
    }

    /// Returns the units-per-em metric of the given font.
    #[inline]
    pub fn units_per_em(&self) -> libc::c_uint {
        unsafe { CTFontGetUnitsPerEm(self as *const _ as _) }
    }

    /// Calculates the advances for an array of glyphs and returns the summed advance.
    pub fn advances_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: &[CGGlyph],
        advances_per_glyph: Option<&mut [CGSize]>,
    ) -> core::ffi::c_double {
        let sink_ptr = advances_per_glyph.map_or_else(null_mut, |x| {
            assert_eq!(x.len(), glyphs.len(), "mismatching count of glyphs and advances");
            x.as_mut_ptr()
        });

        unsafe {
            CTFontGetAdvancesForGlyphs(
                self as *const _ as _,
                orientation,
                glyphs.as_ptr(),
                sink_ptr,
                glyphs.len() as _,
            )
        }
    }

    /// Calculates the bounding rects for an array of glyphs and
    /// returns the overall bounding rectangle for the glyph run.
    pub fn bounding_rects_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: &[CGGlyph],
        bounding_rects_per_glyph: Option<&mut [CGRect]>,
    ) -> CGRect {
        let sink_ptr = bounding_rects_per_glyph.map_or_else(null_mut, |x| {
            assert_eq!(x.len(), glyphs.len(), "mismatching count of glyphs and bounding rects");
            x.as_mut_ptr()
        });

        unsafe {
            CTFontGetBoundingRectsForGlyphs(
                self as *const _ as _,
                orientation,
                glyphs.as_ptr(),
                sink_ptr,
                glyphs.len() as _,
            )
        }
    }
}

/// An opaque type represnting a font descriptor.
pub enum CTFontDescriptor {}
/// A reference to a CTFontDescriptor object.
pub type CTFontDescriptorRef = *mut CTFontDescriptor;
unsafe impl CoreObject for CTFontDescriptor {}
impl CTFontDescriptor {
    pub fn with_attributes(attributes: &CFDictionary) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe {
            CoreRetainedObject::retained_checked(CTFontDescriptorCreateWithAttributes(attributes as *const _ as _))
                .ok_or(())
        }
    }

    pub fn from_data(d: &CFData) -> Option<CoreRetainedObject<Self>> {
        unsafe { CoreRetainedObject::retained_checked(CTFontManagerCreateFontDescriptorFromData(d as *const _ as _)) }
    }
}

/// An opaque type that is used to generate text frames.
pub enum CTFramesetter {}
/// A reference to a CTFramesetter object.
pub type CTFramesetterRef = *mut CTFramesetter;
unsafe impl CoreObject for CTFramesetter {}
impl CTFramesetter {
    /// Creates an immutable framesetter object from an attributed string.
    pub fn new(string: &(impl AsRef<CFAttributedString> + ?Sized)) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe {
            CoreRetainedObject::retained_checked(CTFramesetterCreateWithAttributedString(
                string.as_ref() as *const _ as _
            ))
            .ok_or(())
        }
    }

    /// Determines the frame size needed for a string range.
    pub fn suggest_frame_size_with_constraints(
        &self,
        str_range: impl Into<CFRange>,
        attrs: Option<&CFDictionary>,
        constraints: CGSize,
    ) -> (CGSize, CFRange) {
        let mut fit_range = core::mem::MaybeUninit::uninit();
        let frame_attrs = attrs.map_or_else(null_mut, |p| p as *const _ as _);
        let size = unsafe {
            CTFramesetterSuggestFrameSizeWithConstraints(
                self as *const _ as _,
                str_range.into(),
                frame_attrs,
                constraints,
                fit_range.as_mut_ptr(),
            )
        };

        (size, unsafe { fit_range.assume_init() })
    }

    /// Creates an immutable frame using a framesetter.
    pub fn create_frame(
        &self,
        str_range: impl Into<CFRange>,
        path: &CGPath,
        attributes: Option<&CFDictionary>,
    ) -> Result<CoreRetainedObject<CTFrame>, ()> {
        let a = attributes.map_or_else(null_mut, |p| p as *const _ as _);

        unsafe {
            CoreRetainedObject::retained_checked(CTFramesetterCreateFrame(
                self as *const _ as _,
                str_range.into(),
                path as *const _ as _,
                a,
            ))
            .ok_or(())
        }
    }
}

/// Represents a frame containing multiple lines of text.
pub enum CTFrame {}
/// A reference to a Core Text frame object.
pub type CTFrameRef = *mut CTFrame;
unsafe impl CoreObject for CTFrame {}
impl CTFrame {
    /// Returns an array of lines stored in the frame.
    pub fn lines(&self) -> Result<&CFArray, ()> {
        unsafe { CTFrameGetLines(self as *const _ as _).as_ref().ok_or(()) }
    }

    /// Copies a range of line origins for a frame.
    pub fn line_origins(&self, range: Range<CFIndex>) -> Vec<CGPoint> {
        let mut v = Vec::with_capacity((range.end - range.start) as _);
        unsafe {
            v.set_len((range.end - range.start) as _);
        }
        unsafe {
            CTFrameGetLineOrigins(self as *const _ as _, range.into(), v.as_mut_ptr());
        }

        v
    }
}

/// Represents a line of text.
pub enum CTLine {}
/// A reference to a line object.
pub type CTLineRef = *mut CTLine;
unsafe impl CoreObject for CTLine {}
impl CTLine {
    /// Returns the array of glyph runs that make up the line object.
    pub fn runs(&self) -> Result<&CFArray, ()> {
        unsafe { CTLineGetGlyphRuns(self as *const _ as _).as_ref().ok_or(()) }
    }
}

/// Represents a glyph run, which is a set of consecutive glyphs sharing the same attributes and direction.
pub enum CTRun {}
/// A reference to a run object.
pub type CTRunRef = *mut CTRun;
unsafe impl CoreObject for CTRun {}
impl CTRun {
    /// Gets the glyph count for the run.
    pub fn glyph_count(&self) -> CFIndex {
        unsafe { CTRunGetGlyphCount(self as *const _ as _) }
    }

    /// Copies a range of glyphs and returns an owned buffer filled by `CTRunGetGlyphs`.
    pub fn glyphs(&self, range: Range<CFIndex>) -> Vec<CGGlyph> {
        let mut v = Vec::with_capacity((range.end - range.start) as _);
        unsafe {
            v.set_len((range.end - range.start) as _);
        }
        unsafe {
            CTRunGetGlyphs(self as *const _ as _, range.into(), v.as_mut_ptr());
        }

        v
    }

    /// Returns a slice of a direct pointer for the glyph array stored in the run.
    pub fn glyph_ptr(&self) -> Option<&[CGGlyph]> {
        let count = self.glyph_count();
        let p = unsafe { CTRunGetGlyphsPtr(self as *const _ as _) };

        if p.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(p, count as _) })
        }
    }

    /// Copies a range of glyph positions and returns an owned buffer filled by `CTRunGetGlyphs`.
    pub fn positions(&self, range: Range<CFIndex>) -> Vec<CGPoint> {
        let mut v = Vec::with_capacity((range.end - range.start) as _);
        unsafe {
            v.set_len((range.end - range.start) as _);
        }
        unsafe {
            CTRunGetPositions(self as *const _ as _, range.into(), v.as_mut_ptr());
        }

        v
    }

    /// Returns a slice of a direct pointer for the glyph position array stored in the run.
    pub fn positions_ptr(&self) -> Option<&[CGPoint]> {
        let count = self.glyph_count();
        let p = unsafe { CTRunGetPositionsPtr(self as *const _ as _) };

        if p.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(p, count as _) })
        }
    }

    /// Returns the attribute dictionary that was used to create the glyph run.
    pub fn attributes(&self) -> Result<&CFDictionary, ()> {
        unsafe { CTRunGetAttributes(self as *const _ as _).as_ref().ok_or(()) }
    }

    /// Gets or Copies glyphs:
    /// Equivalent to `self.glyph_ptr().map(Cow::from).unwrap_or_else(|| Cow::from(self.glyphs(0 .. self.glyph_count())))`
    pub fn glyph_array(&self) -> Cow<[CGGlyph]> {
        self.glyph_ptr()
            .map(Cow::from)
            .unwrap_or_else(|| Cow::from(self.glyphs(0..self.glyph_count())))
    }

    /// Gets or Copies glyph positions relative to origin of the line:
    /// Equivalent to `self.position_ptr().map(Cow::from).unwrap_or_else(|| Cow::from(self.positions(0 .. self.glyph_count())))`
    pub fn glyph_rel_positions(&self) -> Cow<[CGPoint]> {
        self.positions_ptr()
            .map(Cow::from)
            .unwrap_or_else(|| Cow::from(self.positions(0..self.glyph_count())))
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "system" {
    fn CTFontCreateWithGraphicsFont(
        graphicsFont: CGFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *mut CTFontDescriptorRef) -> CGFontRef;
    fn CTFontCreateWithFontDescriptor(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
    fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    fn CTFontCopySupportedLanguages(font: CTFontRef) -> CFArrayRef;
    fn CTFontGetGlyphsForCharacters(
        font: CTFontRef,
        characters: *const UniChar,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool;
    fn CTFontCreatePathForGlyph(font: CTFontRef, glyph: CGGlyph, matrix: *const CGAffineTransform) -> CGPathRef;
    fn CTFramesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTFramesetterRef;
    fn CTFramesetterCreateFrame(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        path: CGPathRef,
        frame_attributes: CFDictionaryRef,
    ) -> CTFrameRef;
    fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        frame_attributes: CFDictionaryRef,
        constraints: CGSize,
        fit_range: *mut CFRange,
    ) -> CGSize;
    fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;
    fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
    fn CTFrameGetLineOrigins(frame: CTFrameRef, range: CFRange, origins: *mut CGPoint);

    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
    fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetAdvancesForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> libc::c_double;
    fn CTFontGetBoundingRectsForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        bonding_rects: *mut CGRect,
        count: CFIndex,
    ) -> CGRect;
    fn CTFontGetUnitsPerEm(font: CTFontRef) -> libc::c_uint;

    // CTRun //
    fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *mut CGGlyph);
    fn CTRunGetGlyphsPtr(run: CTRunRef) -> *const CGGlyph;
    fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *mut CGPoint);
    fn CTRunGetPositionsPtr(run: CTRunRef) -> *const CGPoint;
    fn CTRunGetAttributes(run: CTRunRef) -> CFDictionaryRef;

    // CTFontDescriptor //
    fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;

    // CTFontManager //
    fn CTFontManagerCreateFontDescriptorFromData(data: CFDataRef) -> CTFontDescriptorRef;

    // Attributes //
    pub static kCTFontAttributeName: CFStringRef;
    pub static kCTFontFamilyNameAttribute: CFStringRef;
    pub static kCTFontSizeAttribute: CFStringRef;
    pub static kCTKernAttributeName: CFStringRef;
    pub static kCTLanguageAttributeName: CFStringRef;
    pub static kCTFontTraitsAttribute: CFStringRef;

    // Traits //
    pub static kCTFontSymbolicTrait: CFStringRef;
    pub static kCTFontWeightTrait: CFStringRef;
}

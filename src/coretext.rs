//! Core Text

use crate::{
    opt_pointer, opt_pointer_mut, CFArray, CFArrayRef, CFAttributedString, CFAttributedStringRef, CFData, CFDataRef,
    CFDictionary, CFDictionaryRef, CFIndex, CFRange, CFStringRef, CGAffineTransform, CGFloat, CGFont, CGFontRef,
    CGGlyph, CGPath, CGPathRef, CGPoint, CGRect, CGSize, CoreRetainedObject, NSFont, UniChar,
};
use core::ptr::null_mut;
use core::slice;

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum CTFontSymbolicTraits {
    ItalicTrait = 1 << 0,
    BoldTrait = 1 << 1,
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum CTFontOrientation {
    Default = 0,
    Horizontal,
    Vertical,
}

DefineCoreObject! {
    /// An opaque type represents a Core Text font object.
    pub CTFont;
}
/// A reference to a Core Text font object.
pub type CTFontRef = *const CTFont;
TollfreeBridge!(NSFont = CTFont);
impl CTFont {
    /// Creates a new font reference from an existing Core Graphics font reference.
    #[inline(always)]
    pub fn from_cg(
        graphics_font: &mut CGFont,
        size: CGFloat,
        matrix: Option<&CGAffineTransform>,
        attributes: Option<&mut CTFontDescriptor>,
    ) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe {
            CoreRetainedObject::retained(CTFontCreateWithGraphicsFont(
                graphics_font,
                size,
                opt_pointer(matrix),
                opt_pointer_mut(attributes),
            ))
            .ok_or(())
        }
    }

    /// Returns a Core Graphics font reference.
    #[inline(always)]
    pub fn to_cg(&self) -> Result<CoreRetainedObject<CGFont>, ()> {
        unsafe { CoreRetainedObject::retained(CTFontCopyGraphicsFont(self, null_mut())).ok_or(()) }
    }

    /// Returns a Core Graphics font reference and attributes.
    pub fn to_cg_with_attributes(
        &self,
    ) -> Result<(CoreRetainedObject<CGFont>, CoreRetainedObject<CTFontDescriptor>), ()> {
        let mut attrs = core::mem::MaybeUninit::uninit();
        let font_ptr = unsafe { CTFontCopyGraphicsFont(self, attrs.as_mut_ptr()) };

        Ok((unsafe { CoreRetainedObject::retained(font_ptr).ok_or(())? }, unsafe {
            CoreRetainedObject::retained(attrs.assume_init()).ok_or(())?
        }))
    }

    /// Returns a new font reference that best matches the given font descriptor
    #[inline(always)]
    pub fn from_font_descriptor(
        descriptor: &CTFontDescriptor,
        size: CGFloat,
        matrix: Option<&CGAffineTransform>,
    ) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe {
            CoreRetainedObject::retained(CTFontCreateWithFontDescriptor(descriptor, size, opt_pointer(matrix)))
                .ok_or(())
        }
    }

    /// Returns an array of languages supported by the font.
    #[inline(always)]
    pub fn supported_languages(&self) -> Result<CoreRetainedObject<CFArray>, ()> {
        unsafe { CoreRetainedObject::retained(CTFontCopySupportedLanguages(self)).ok_or(()) }
    }

    /// Provides basic Unicode encoding for the given font, returning by reference an array of `CGGlyph` values
    /// corresponding to a given array of Unicode characters for the given font.
    pub fn glyphs_for_characters(&self, characters: &[UniChar]) -> Result<Vec<CGGlyph>, ()> {
        let mut glyphs = Vec::with_capacity(characters.len());
        unsafe {
            glyphs.set_len(characters.len());
        }
        let r = unsafe {
            CTFontGetGlyphsForCharacters(self, characters.as_ptr(), glyphs.as_mut_ptr(), characters.len() as _)
        };

        if !r {
            Err(())
        } else {
            Ok(glyphs)
        }
    }

    /// Creates a path for the specified glyph.
    #[inline(always)]
    pub fn create_path_for_glyph(
        &self,
        glyph: CGGlyph,
        transform: Option<&CGAffineTransform>,
    ) -> Result<CoreRetainedObject<CGPath>, ()> {
        unsafe { CoreRetainedObject::retained(CTFontCreatePathForGlyph(self, glyph, opt_pointer(transform))).ok_or(()) }
    }

    /// Returns a new font with additional attributes based on the original font.
    #[inline(always)]
    pub fn create_copy_with_attributes(
        &self,
        size: CGFloat,
        transform: Option<&CGAffineTransform>,
        attributes: Option<&mut CTFontDescriptor>,
    ) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe {
            CoreRetainedObject::retained(CTFontCreateCopyWithAttributes(
                self,
                size,
                opt_pointer(transform),
                opt_pointer_mut(attributes),
            ))
            .ok_or(())
        }
    }
}

/// Font Metrics
impl CTFont {
    /// Returns the point size of the font.
    #[inline(always)]
    pub fn size(&self) -> CGFloat {
        unsafe { CTFontGetSize(self) }
    }

    /// Returns the scaled font-ascent metric of the font.
    #[inline(always)]
    pub fn ascent(&self) -> CGFloat {
        unsafe { CTFontGetAscent(self) }
    }

    /// Returns the scaled font-descent metric of the font.
    #[inline(always)]
    pub fn descent(&self) -> CGFloat {
        unsafe { CTFontGetDescent(self) }
    }

    /// Returns the cap-height metric of the font.
    #[inline(always)]
    pub fn cap_height(&self) -> CGFloat {
        unsafe { CTFontGetCapHeight(self) }
    }

    /// Returns the x-height metric of the font.
    #[inline(always)]
    pub fn x_height(&self) -> CGFloat {
        unsafe { CTFontGetXHeight(self) }
    }

    /// Returns the units-per-em metric of the given font.
    #[inline(always)]
    pub fn units_per_em(&self) -> libc::c_uint {
        unsafe { CTFontGetUnitsPerEm(self) }
    }

    /// Calculates the advances for an array of glyphs and returns the summed advance.
    pub fn advances_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: &[CGGlyph],
        advances_per_glyph: Option<&mut [CGSize]>,
    ) -> core::ffi::c_double {
        let sink_ptr = match advances_per_glyph {
            Some(x) => {
                assert_eq!(x.len(), glyphs.len(), "mismatching count of glyphs and advances");

                x.as_mut_ptr()
            }
            None => core::ptr::null_mut(),
        };

        unsafe { CTFontGetAdvancesForGlyphs(self, orientation, glyphs.as_ptr(), sink_ptr, glyphs.len() as _) }
    }

    /// Calculates the bounding rects for an array of glyphs and
    /// returns the overall bounding rectangle for the glyph run.
    pub fn bounding_rects_for_glyphs(
        &self,
        orientation: CTFontOrientation,
        glyphs: &[CGGlyph],
        bounding_rects_per_glyph: Option<&mut [CGRect]>,
    ) -> CGRect {
        let sink_ptr = match bounding_rects_per_glyph {
            Some(x) => {
                assert_eq!(x.len(), glyphs.len(), "mismatching count of glyphs and bounding rects");

                x.as_mut_ptr()
            }
            None => core::ptr::null_mut(),
        };

        unsafe { CTFontGetBoundingRectsForGlyphs(self, orientation, glyphs.as_ptr(), sink_ptr, glyphs.len() as _) }
    }
}

DefineCoreObject! {
    /// An opaque type representing a font descriptor.
    pub CTFontDescriptor;
}
/// A reference to a CTFontDescriptor object.
pub type CTFontDescriptorRef = *const CTFontDescriptor;
impl CTFontDescriptor {
    #[inline(always)]
    pub fn with_attributes(attributes: &CFDictionary) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe { CoreRetainedObject::retained(CTFontDescriptorCreateWithAttributes(attributes)).ok_or(()) }
    }

    #[inline(always)]
    pub fn from_data(d: &CFData) -> Option<CoreRetainedObject<Self>> {
        unsafe { CoreRetainedObject::retained(CTFontManagerCreateFontDescriptorFromData(d)) }
    }
}

DefineCoreObject! {
    /// An opaque type that is used to generate text frames.
    pub CTFramesetter;
}
/// A reference to a CTFramesetter object.
pub type CTFramesetterRef = *const CTFramesetter;
impl CTFramesetter {
    /// Creates an immutable framesetter object from an attributed string.
    #[inline(always)]
    pub fn new(string: &CFAttributedString) -> Result<CoreRetainedObject<Self>, ()> {
        unsafe { CoreRetainedObject::retained(CTFramesetterCreateWithAttributedString(string)).ok_or(()) }
    }

    /// Determines the frame size needed for a string range.
    pub fn suggest_frame_size_with_constraints(
        &self,
        str_range: impl Into<CFRange>,
        attrs: Option<&CFDictionary>,
        constraints: CGSize,
    ) -> (CGSize, CFRange) {
        let mut fit_range = core::mem::MaybeUninit::uninit();
        let size = unsafe {
            CTFramesetterSuggestFrameSizeWithConstraints(
                self,
                str_range.into(),
                opt_pointer(attrs),
                constraints,
                fit_range.as_mut_ptr(),
            )
        };

        (size, unsafe { fit_range.assume_init() })
    }

    /// Creates an immutable frame using a framesetter.
    #[inline(always)]
    pub fn create_frame(
        &self,
        str_range: CFRange,
        path: &CGPath,
        attributes: Option<&CFDictionary>,
    ) -> Result<CoreRetainedObject<CTFrame>, ()> {
        unsafe {
            CoreRetainedObject::retained(CTFramesetterCreateFrame(self, str_range, path, opt_pointer(attributes)))
                .ok_or(())
        }
    }
}

DefineCoreObject! {
    /// Represents a frame containing multiple lines of text.
    pub CTFrame;
}
/// A reference to a Core Text frame object.
pub type CTFrameRef = *const CTFrame;
impl CTFrame {
    /// Returns an array of lines stored in the frame.
    #[inline(always)]
    pub fn lines(&self) -> Result<&CFArray, ()> {
        unsafe { CTFrameGetLines(self).as_ref().ok_or(()) }
    }

    /// Copies a range of line origins for a frame.
    #[inline(always)]
    pub fn line_origins(&self, start: CFIndex, sink: &mut [CGPoint]) {
        unsafe {
            CTFrameGetLineOrigins(
                self,
                CFRange {
                    location: start,
                    length: sink.len() as _,
                },
                sink.as_mut_ptr(),
            );
        }
    }
}

DefineCoreObject! {
    /// Represents a line of text.
    pub CTLine;
}
/// A reference to a line object.
pub type CTLineRef = *const CTLine;
impl CTLine {
    /// Returns the array of glyph runs that make up the line object.
    #[inline(always)]
    pub fn runs(&self) -> Result<&CFArray, ()> {
        unsafe { CTLineGetGlyphRuns(self).as_ref().ok_or(()) }
    }
}

DefineCoreObject! {
    /// Represents a glyph run, which is a set of consecutive glyphs sharing the same attributes and direction.
    pub CTRun;
}
/// A reference to a run object.
pub type CTRunRef = *const CTRun;
impl CTRun {
    /// Gets the glyph count for the run.
    #[inline(always)]
    pub fn glyph_count(&self) -> CFIndex {
        unsafe { CTRunGetGlyphCount(self) }
    }

    /// Copies a range of glyphs and returns an owned buffer filled by `CTRunGetGlyphs`.
    #[inline(always)]
    pub fn glyphs(&self, start: CFIndex, sink: &mut [CGGlyph]) {
        unsafe {
            CTRunGetGlyphs(
                self,
                CFRange {
                    location: start,
                    length: sink.len() as _,
                },
                sink.as_mut_ptr(),
            );
        }
    }

    /// Returns a direct pointer for the glyph array stored in the run.
    #[inline(always)]
    pub fn glyph_ptr(&self) -> *const CGGlyph {
        unsafe { CTRunGetGlyphsPtr(self) }
    }

    /// Returns a slice of a direct pointer for the glyph array stored in the run.
    #[inline]
    pub fn glyph_ref(&self) -> Option<&[CGGlyph]> {
        let (count, p) = (self.glyph_count(), self.glyph_ptr());

        if p.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(p, count as _) })
        }
    }

    /// Copies a range of glyph positions and returns an owned buffer filled by `CTRunGetGlyphs`.
    #[inline(always)]
    pub fn positions(&self, start: CFIndex, sink: &mut [CGPoint]) {
        unsafe {
            CTRunGetPositions(
                self,
                CFRange {
                    location: start,
                    length: sink.len() as _,
                },
                sink.as_mut_ptr(),
            );
        }
    }

    /// Returns a direct pointer for the glyph position array stored in the run.
    #[inline(always)]
    pub fn positions_ptr(&self) -> *const CGPoint {
        unsafe { CTRunGetPositionsPtr(self) }
    }

    /// Returns a slice of a direct pointer for the glyph position array stored in the run.
    #[inline]
    pub fn positions_ref(&self) -> Option<&[CGPoint]> {
        let (count, p) = (self.glyph_count(), self.positions_ptr());

        if p.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(p, count as _) })
        }
    }

    /// Returns the attribute dictionary that was used to create the glyph run.
    #[inline(always)]
    pub fn attributes(&self) -> Result<&CFDictionary, ()> {
        unsafe { CTRunGetAttributes(self).as_ref().ok_or(()) }
    }
}

#[link(name = "CoreText", kind = "framework")]
unsafe extern "system" {
    unsafe fn CTFontCreateWithGraphicsFont(
        graphicsFont: CGFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    unsafe fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *mut CTFontDescriptorRef) -> CGFontRef;
    unsafe fn CTFontCreateWithFontDescriptor(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
    unsafe fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    unsafe fn CTFontCopySupportedLanguages(font: CTFontRef) -> CFArrayRef;
    unsafe fn CTFontGetGlyphsForCharacters(
        font: CTFontRef,
        characters: *const UniChar,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool;
    unsafe fn CTFontCreatePathForGlyph(font: CTFontRef, glyph: CGGlyph, matrix: *const CGAffineTransform) -> CGPathRef;
    unsafe fn CTFramesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTFramesetterRef;
    unsafe fn CTFramesetterCreateFrame(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        path: CGPathRef,
        frame_attributes: CFDictionaryRef,
    ) -> CTFrameRef;
    unsafe fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        frame_attributes: CFDictionaryRef,
        constraints: CGSize,
        fit_range: *mut CFRange,
    ) -> CGSize;
    unsafe fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;
    unsafe fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
    unsafe fn CTFrameGetLineOrigins(frame: CTFrameRef, range: CFRange, origins: *mut CGPoint);

    unsafe fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    unsafe fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
    unsafe fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
    unsafe fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    unsafe fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    unsafe fn CTFontGetAdvancesForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> core::ffi::c_double;
    unsafe fn CTFontGetBoundingRectsForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        bonding_rects: *mut CGRect,
        count: CFIndex,
    ) -> CGRect;
    unsafe fn CTFontGetUnitsPerEm(font: CTFontRef) -> core::ffi::c_uint;

    // CTRun //
    unsafe fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    unsafe fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *mut CGGlyph);
    unsafe fn CTRunGetGlyphsPtr(run: CTRunRef) -> *const CGGlyph;
    unsafe fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *mut CGPoint);
    unsafe fn CTRunGetPositionsPtr(run: CTRunRef) -> *const CGPoint;
    unsafe fn CTRunGetAttributes(run: CTRunRef) -> CFDictionaryRef;

    // CTFontDescriptor //
    unsafe fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;

    // CTFontManager //
    unsafe fn CTFontManagerCreateFontDescriptorFromData(data: CFDataRef) -> CTFontDescriptorRef;

    // Attributes //
    pub unsafe static kCTFontAttributeName: CFStringRef;
    pub unsafe static kCTFontFamilyNameAttribute: CFStringRef;
    pub unsafe static kCTFontSizeAttribute: CFStringRef;
    pub unsafe static kCTKernAttributeName: CFStringRef;
    pub unsafe static kCTLanguageAttributeName: CFStringRef;
    pub unsafe static kCTFontTraitsAttribute: CFStringRef;

    // Traits //
    pub unsafe static kCTFontSymbolicTrait: CFStringRef;
    pub unsafe static kCTFontWeightTrait: CFStringRef;
}

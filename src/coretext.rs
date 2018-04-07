//! Core Text

use {ExternalRc, ExternalRced};
use std::ptr::{null, null_mut};
use std::ops::Range;
use std::slice;
use std::borrow::Cow;

/// An opaque type represents a Core Text font object.
pub enum CTFont {}
/// A reference to a Core Text font object.
pub type CTFontRef = *mut CTFont;

/// toll-free bridging
impl AsRef<CTFont> for ::NSFont { fn as_ref(&self) -> &CTFont { unsafe { ::std::mem::transmute(self) } } }
impl AsRef<::NSFont> for CTFont { fn as_ref(&self) -> &::NSFont { unsafe { ::std::mem::transmute(self) } } }
impl ExternalRced for CTFont {
    unsafe fn own_from_unchecked(p: *mut Self) -> ExternalRc<Self> {
        ExternalRc::with_fn(p, ::cfretain::<Self>, ::cfrelease::<Self>)
    }
}
impl CTFont {
    /// Creates a new font reference from an existing Core Graphics font reference.
    pub fn from_cg(graphics_font: &::CGFont, size: ::CGFloat, matrix: Option<&::CGAffineTransform>,
            attributes: Option<&CTFontDescriptor>) -> Result<ExternalRc<Self>, ()> {
        let matrix_ref = matrix.map_or(null(), |p| p as _);
        let attrs_ref = attributes.map_or(null_mut(), |p| p as *const _ as _);
        unsafe {
            Self::own_from(CTFontCreateWithGraphicsFont(graphics_font as *const _ as _, size, matrix_ref, attrs_ref))
                .ok_or(())
        }
    }
    /// Returns a Core Graphics font reference.
    pub fn to_cg(&self) -> Result<ExternalRc<::CGFont>, ()> {
        unsafe { ::CGFont::own_from(CTFontCopyGraphicsFont(self as *const _ as _, null_mut())).ok_or(()) }
    }
    /// Returns a Core Graphics font reference and attributes.
    pub fn to_cg_with_attributes(&self) -> Result<(ExternalRc<::CGFont>, ExternalRc<CTFontDescriptor>), ()> {
        let mut attrs = null_mut();
        let font_ptr = unsafe { CTFontCopyGraphicsFont(self as *const _ as _, &mut attrs) };
        unsafe { ::CGFont::own_from(font_ptr).and_then(|f| CTFontDescriptor::own_from(attrs).map(move |d| (f, d))).ok_or(()) }
    }
    
    /// Returns an array of languages supported by the font.
    pub fn supported_languages(&self) -> Result<ExternalRc<::CFArray>, ()> {
        unsafe { ::CFArray::own_from(CTFontCopySupportedLanguages(self as *const _ as _)).ok_or(()) }
    }
    /// Provides basic Unicode encoding for the given font, returning by reference an array of `CGGlyph` values
    /// corresponding to a given array of Unicode characters for the given font.
    pub fn glyphs_for_characters(&self, characters: &[::UniChar]) -> Result<Vec<::CGGlyph>, ()> {
        let mut glyphs = Vec::with_capacity(characters.len()); unsafe { glyphs.set_len(characters.len()); }
        let r = unsafe {
            CTFontGetGlyphsForCharacters(self as *const _ as _,
                characters.as_ptr(), glyphs.as_mut_ptr(), characters.len() as _)
        };
        if !r { Err(()) } else { Ok(glyphs) }
    }
    /// Creates a path for the specified glyph.
    pub fn create_path_for_glyph(&self, glyph: ::CGGlyph, transform: Option<&::CGAffineTransform>)
            -> Result<::ExternalRc<::CGPath>, ()> {
        let ptf = transform.map_or(null(), |p| p as _);
        return unsafe { ::CGPath::own_from(CTFontCreatePathForGlyph(self as *const _ as _, glyph, ptf)).ok_or(()) };
    }
}

/// An opaque type represnting a font descriptor.
pub enum CTFontDescriptor {}
/// A reference to a CTFontDescriptor object.
pub type CTFontDescriptorRef = *mut CTFontDescriptor;
impl ExternalRced for CTFontDescriptor {
    unsafe fn own_from_unchecked(p: *mut Self) -> ExternalRc<Self> {
        ExternalRc::with_fn(p, ::cfretain::<Self>, ::cfrelease::<Self>)
    }
}

/// An opaque type that is used to generate text frames.
pub enum CTFramesetter {}
/// A reference to a CTFramesetter object.
pub type CTFramesetterRef = *mut CTFramesetter;
impl ExternalRced for CTFramesetter {
    unsafe fn own_from_unchecked(p: *mut Self) -> ExternalRc<Self> {
        ExternalRc::with_fn(p, ::cfretain::<Self>, ::cfrelease::<Self>)
    }
}
impl CTFramesetter {
    /// Creates an immutable framesetter object from an attributed string.
    pub fn new<A: AsRef<::CFAttributedString> + ?Sized>(string: &A) -> Result<ExternalRc<Self>, ()> {
        unsafe { Self::own_from(CTFramesetterCreateWithAttributedString(string.as_ref() as *const _ as _)).ok_or(()) }
    }
    /// Determines the frame size needed for a string range.
    pub fn suggest_frame_size_with_constraints<R: Into<::CFRange>>(&self, str_range: R, attrs: Option<&::CFDictionary>,
            constraints: ::CGSize) -> (::CGSize, ::CFRange) {
        let mut fit_range = unsafe { ::std::mem::uninitialized() };
        let frame_attrs = attrs.map_or(null_mut(), |p| p as *const _ as _);
        let size = unsafe {
            CTFramesetterSuggestFrameSizeWithConstraints(self as *const _ as _, str_range.into(), frame_attrs,
                constraints, &mut fit_range)
        };
        return (size, fit_range);
    }
}

/// Represents a frame containing multiple lines of text.
pub enum CTFrame {}
/// A reference to a Core Text frame object.
pub type CTFrameRef = *mut CTFrame;
impl ExternalRced for CTFrame {
    unsafe fn own_from_unchecked(p: *mut Self) -> ExternalRc<Self> {
        ExternalRc::with_fn(p, ::cfretain::<Self>, ::cfrelease::<Self>)
    }
}
impl CTFramesetter {
    /// Creates an immutable frame using a framesetter.
    pub fn create_frame<R: Into<::CFRange>>(&self, str_range: R, path: &::CGPath, attributes: Option<&::CFDictionary>)
            -> Result<ExternalRc<CTFrame>, ()> {
        let a = attributes.map_or(null_mut(), |p| p as *const _ as _);
        unsafe {
            CTFrame::own_from(CTFramesetterCreateFrame(self as *const _ as _, str_range.into(), path as *const _ as _, a))
                .ok_or(())
        }
    }
}

/// Represents a line of text.
pub enum CTLine {}
/// A reference to a line object.
pub type CTLineRef = *mut CTLine;
impl CTFrame {
    /// Returns an array of lines stored in the frame.
    pub fn lines(&self) -> Result<&::CFArray, ()> {
        unsafe { CTFrameGetLines(self as *const _ as _).as_ref().ok_or(()) }
    }
    /// Copies a range of line origins for a frame.
    pub fn line_origins(&self, range: Range<::CFIndex>) -> Vec<::CGPoint> {
        let mut v = Vec::with_capacity((range.end - range.start) as _);
        unsafe { v.set_len((range.end - range.start) as _); }
        unsafe { CTFrameGetLineOrigins(self as *const _ as _, range.into(), v.as_mut_ptr()); }
        return v;
    }
}

/// Represents a glyph run, which is a set of consecutive glyphs sharing the same attributes and direction.
pub enum CTRun {}
/// A reference to a run object.
pub type CTRunRef = *mut CTRun;
impl CTLine {
    /// Returns the array of glyph runs that make up the line object.
    pub fn runs(&self) -> Result<&::CFArray, ()> {
        unsafe { CTLineGetGlyphRuns(self as *const _ as _).as_ref().ok_or(()) }
    }
}
impl CTRun {
    /// Gets the glyph count for the run.
    pub fn glyph_count(&self) -> ::CFIndex { unsafe { CTRunGetGlyphCount(self as *const _ as _) } }
    /// Copies a range of glyphs and returns an owned buffer filled by `CTRunGetGlyphs`.
    pub fn glyphs(&self, range: Range<::CFIndex>) -> Vec<::CGGlyph> {
        let mut v = Vec::with_capacity((range.end - range.start) as _);
        unsafe { v.set_len((range.end - range.start) as _); }
        unsafe { CTRunGetGlyphs(self as *const _ as _, range.into(), v.as_mut_ptr()); }
        return v;
    }
    /// Returns a slice of a direct pointer for the glyph array stored in the run.
    pub fn glyph_ptr(&self) -> Option<&[::CGGlyph]> {
        let count = self.glyph_count();
        let p = unsafe { CTRunGetGlyphsPtr(self as *const _ as _) };
        return if p.is_null() { None } else { Some(unsafe { slice::from_raw_parts(p, count as _) }) };
    }
    /// Copies a range of glyph positions and returns an owned buffer filled by `CTRunGetGlyphs`.
    pub fn positions(&self, range: Range<::CFIndex>) -> Vec<::CGPoint> {
        let mut v = Vec::with_capacity((range.end - range.start) as _);
        unsafe { v.set_len((range.end - range.start) as _); }
        unsafe { CTRunGetPositions(self as *const _ as _, range.into(), v.as_mut_ptr()); }
        return v;
    }
    /// Returns a slice of a direct pointer for the glyph position array stored in the run.
    pub fn position_ptr(&self) -> Option<&[::CGPoint]> {
        let count = self.glyph_count();
        let p = unsafe { CTRunGetPositionsPtr(self as *const _ as _) };
        return if p.is_null() { None } else { Some(unsafe { slice::from_raw_parts(p, count as _) }) };
    }
    /// Returns the attribute dictionary that was used to create the glyph run.
    pub fn attributes(&self) -> Result<&::CFDictionary, ()> {
        unsafe { (CTRunGetAttributes(self as *const _ as _)).as_ref().ok_or(()) }
    }

    /// Gets or Copies glyphs:
    /// Equivalent to `self.glyph_ptr().map(Cow::from).unwrap_or_else(|| Cow::from(self.glyphs(0 .. self.glyph_count())))`
    pub fn glyph_array(&self) -> Cow<[::CGGlyph]> {
        self.glyph_ptr().map(Cow::from).unwrap_or_else(|| Cow::from(self.glyphs(0 .. self.glyph_count())))
    }
    /// Gets or Copies glyph positions relative to origin of the line:
    /// Equivalent to `self.position_ptr().map(Cow::from).unwrap_or_else(|| Cow::from(self.positions(0 .. self.glyph_count())))`
    pub fn glyph_rel_positions(&self) -> Cow<[::CGPoint]> {
        self.position_ptr().map(Cow::from).unwrap_or_else(|| Cow::from(self.positions(0 .. self.glyph_count())))
    }
}

#[link(name = "CoreText", kind = "framework")] extern "system" {
    fn CTFontCreateWithGraphicsFont(graphicsFont: ::CGFontRef, size: ::CGFloat, matrix: *const ::CGAffineTransform,
        attributes: CTFontDescriptorRef) -> CTFontRef;
    fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *mut CTFontDescriptorRef) -> ::CGFontRef;
    fn CTFontCopySupportedLanguages(font: CTFontRef) -> ::CFArrayRef;
    fn CTFontGetGlyphsForCharacters(font: CTFontRef, characters: *const ::UniChar, glyphs: *mut ::CGGlyph,
        count: ::CFIndex) -> bool;
    fn CTFontCreatePathForGlyph(font: CTFontRef, glyph: ::CGGlyph, matrix: *const ::CGAffineTransform)
        -> ::CGPathRef;
    fn CTFramesetterCreateWithAttributedString(string: ::CFAttributedStringRef) -> CTFramesetterRef;
    fn CTFramesetterCreateFrame(framesetter: CTFramesetterRef, string_range: ::CFRange, path: ::CGPathRef,
        frame_attributes: ::CFDictionaryRef) -> CTFrameRef;
    fn CTFramesetterSuggestFrameSizeWithConstraints(framesetter: CTFramesetterRef, string_range: ::CFRange,
        frame_attributes: ::CFDictionaryRef, constraints: ::CGSize, fit_range: *mut ::CFRange) -> ::CGSize;
    fn CTFrameGetLines(frame: CTFrameRef) -> ::CFArrayRef;
    fn CTLineGetGlyphRuns(line: CTLineRef) -> ::CFArrayRef;
    fn CTFrameGetLineOrigins(frame: CTFrameRef, range: ::CFRange, origins: *mut ::CGPoint);

    // CTRun //
    fn CTRunGetGlyphCount(run: CTRunRef) -> ::CFIndex;
    fn CTRunGetGlyphs(run: CTRunRef, range: ::CFRange, buffer: *mut ::CGGlyph);
    fn CTRunGetGlyphsPtr(run: CTRunRef) -> *const ::CGGlyph;
    fn CTRunGetPositions(run: CTRunRef, range: ::CFRange, buffer: *mut ::CGPoint);
    fn CTRunGetPositionsPtr(run: CTRunRef) -> *const ::CGPoint;
    fn CTRunGetAttributes(run: CTRunRef) -> ::CFDictionaryRef;

    // Attributes //
    pub static kCTFontAttributeName: ::CFStringRef;
    pub static kCTLanguageAttributeName: ::CFStringRef;
}

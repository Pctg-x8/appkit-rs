//! Core Text

use {ExternalRc, ExternalRced};
use std::ptr::{null, null_mut};

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
}

/// An opaque type represnting a font descriptor.
pub enum CTFontDescriptor {}
/// A reference to a CTFontDescriptor object.
pub type CTFontDescriptorRef = *mut CTFontDescriptor;

#[link(name = "CoreText", kind = "framework")] extern "system" {
    fn CTFontCreateWithGraphicsFont(graphicsFont: ::CGFontRef, size: ::CGFloat, matrix: *const ::CGAffineTransform,
        attributes: CTFontDescriptorRef) -> CTFontRef;
    fn CTFontCopySupportedLanguages(font: CTFontRef) -> ::CFArrayRef;
    fn CTFontGetGlyphsForCharacters(font: CTFontRef, characters: *const ::UniChar, glyphs: *mut ::CGGlyph,
        count: ::CFIndex) -> bool;
}

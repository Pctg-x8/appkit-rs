//! Core Foundation

use libc::*;
use {ExternalRc, ExternalRced};

/// An untyped "generic" reference to any Core Foundation object.
pub type CFTypeRef = *const c_void;

/// Retains a Core Foundation object.
pub unsafe extern "system" fn cfretain<T>(cf: *mut T) -> *mut T { CFRetain(cf as *const _) as *mut _ }
/// Releases a Core Foundation object.
pub unsafe extern "system" fn cfrelease<T>(cf: *mut T) { CFRelease(cf as *const _) }

/// Priority values used for kAXPriorityKey.
pub type CFIndex = c_long;
/// Manages static ordered collections of values.
pub enum CFArray {}
/// A reference to an immutable array object.
pub type CFArrayRef = *mut CFArray;
impl ExternalRced for CFArray {
    unsafe fn own_from_unchecked(r: *mut Self) -> ExternalRc<Self> {
        ExternalRc::with_fn(r, cfretain::<Self>, cfrelease::<Self>)
    }
}
impl CFArray {
    /// Returns the number of values currently in an array.
    pub fn len(&self) -> CFIndex { unsafe { CFArrayGetCount(self as *const _ as _) } }
    /// Retrieves a value at a given index.
    pub unsafe fn get<T>(&self, idx: CFIndex) -> Option<&T> {
        (CFArrayGetValueAtIndex(self as *const _ as _, idx) as *const T).as_ref()
    }
}
/// toll-free bridging
impl<T: ::ObjcObjectBase> AsRef<::NSArray<T>> for CFArray {
    fn as_ref(&self) -> &::NSArray<T> { unsafe { ::std::mem::transmute(self) } }
}
impl<T: ::ObjcObjectBase> AsRef<CFArray> for ::NSArray<T> {
    fn as_ref(&self) -> &CFArray { unsafe { ::std::mem::transmute(self) } }
}
/// Manages associations of key-value pairs.
pub enum CFDictionary {}
/// A reference to an immutable dictionary object.
pub type CFDictionaryRef = *mut CFDictionary;
/// toll-free bridging
impl<K: ::ObjcObjectBase, V: ::ObjcObjectBase> AsRef<::NSDictionary<K, V>> for CFDictionary {
    fn as_ref(&self) -> &::NSDictionary<K, V> { unsafe { ::std::mem::transmute(self) } }
}
impl<K: ::ObjcObjectBase, V: ::ObjcObjectBase> AsRef<CFDictionary> for ::NSDictionary<K, V> {
    fn as_ref(&self) -> &CFDictionary { unsafe { ::std::mem::transmute(self) } }
}
impl CFDictionary {
    /// Returns the value associated with a given key.
    pub unsafe fn get<K, T>(&self, key: &K) -> Option<&T> {
        (CFDictionaryGetValue(self as *const _ as _, key as *const K as _) as *const T).as_ref()
    }
}

/// Manages character strings and associated sets of attributes.
pub enum CFAttributedString {}
/// A reference to a CFAttributedString object.
pub type CFAttributedStringRef = *mut CFAttributedString;
TollfreeBridge!(CFAttributedString = ::NSAttributedString);

/// A structure representing a range of sequential items in a container.
#[repr(C)] #[derive(Clone, Debug, PartialEq, Eq)]
pub struct CFRange { location: CFIndex, length: CFIndex }
use std::ops::{Range, RangeTo, RangeFrom, RangeFull};
impl From<Range<u32>> for CFRange {
    fn from(r: Range<u32>) -> Self { CFRange { location: r.start as _, length: r.len() as _ } }
}
impl From<Range<CFIndex>> for CFRange {
    fn from(r: Range<CFIndex>) -> Self { CFRange { location: r.start, length: r.end - r.start } }
}
impl From<RangeFrom<u32>> for CFRange {
    fn from(r: RangeFrom<u32>) -> Self { CFRange { location: r.start as _, length: CFIndex::max_value() } }
}
impl From<RangeTo<u32>> for CFRange { fn from(r: RangeTo<u32>) -> Self { CFRange { location: 0, length: r.end as _ } } }
impl From<RangeFull> for CFRange {
    fn from(_: RangeFull) -> Self { CFRange { location: 0, length: 0 } }
}

pub enum CFString {}
/// A reference to a CFString object.
pub type CFStringRef = *mut CFString;
TollfreeBridge!(CFString = ::NSString);

pub enum CFNumber {}
/// A reference to a CFNumber object.
pub type CFNumberRef = *mut CFNumber;
TollfreeBridge!(CFNumber = ::NSNumber);

#[link(name = "CoreFoundation", kind = "framework")] extern "system" {
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);
    fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> *const c_void;
    fn CFDictionaryGetValue(dict: CFDictionaryRef, key: *const c_void) -> *const c_void;
}

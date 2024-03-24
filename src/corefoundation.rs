//! Core Foundation

use crate::{NSArray, NSAttributedString, NSDictionary, NSNumber, NSString};
use libc::*;
use objc_ext::ObjcObject;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

/// An untyped "generic" reference to any Core Foundation object.
pub type CFTypeRef = *const c_void;

/// Retains a Core Foundation object.
pub unsafe extern "system" fn cfretain<T>(cf: *mut T) -> *mut T {
    CFRetain(cf as *const _) as *mut _
}
/// Releases a Core Foundation object.
pub unsafe extern "system" fn cfrelease<T>(cf: *mut T) {
    CFRelease(cf as *const _)
}

pub unsafe trait CoreObject {}
/// autorelease box for CoreFoundation objects
#[repr(transparent)]
pub struct CoreRetainedObject<T: CoreObject>(core::ptr::NonNull<T>);
impl<T: CoreObject> CoreRetainedObject<T> {
    #[inline(always)]
    pub(crate) unsafe fn retained(ptr: core::ptr::NonNull<T>) -> Self {
        Self(ptr)
    }

    #[inline]
    pub(crate) unsafe fn retained_checked(ptr: *mut T) -> Option<Self> {
        core::ptr::NonNull::new(ptr).map(Self)
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    #[inline]
    pub fn try_retain(&self) -> Option<Self> {
        core::ptr::NonNull::new(unsafe { cfretain(self.as_ptr()) }).map(Self)
    }
}
impl<T: CoreObject> Clone for CoreRetainedObject<T> {
    #[inline]
    fn clone(&self) -> Self {
        self.try_retain().expect("Retaining CoreFoundation object")
    }
}
impl<T: CoreObject> Drop for CoreRetainedObject<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { cfrelease(self.as_ptr()) }
    }
}
impl<T: CoreObject> core::ops::Deref for CoreRetainedObject<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T: CoreObject> core::ops::DerefMut for CoreRetainedObject<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T: CoreObject> AsRef<T> for CoreRetainedObject<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: CoreObject> AsMut<T> for CoreRetainedObject<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}
impl<T: CoreObject> core::borrow::Borrow<T> for CoreRetainedObject<T> {
    fn borrow(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: CoreObject> core::borrow::BorrowMut<T> for CoreRetainedObject<T> {
    fn borrow_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

/// Priority values used for kAXPriorityKey.
pub type CFIndex = c_long;
/// Manages static ordered collections of values.
pub enum CFArray {}
/// A reference to an immutable array object.
pub type CFArrayRef = *mut CFArray;
unsafe impl CoreObject for CFArray {}
impl CFArray {
    /// Returns the number of values currently in an array.
    pub fn len(&self) -> CFIndex {
        unsafe { CFArrayGetCount(self as *const _ as _) }
    }

    /// Retrieves a value at a given index.
    pub unsafe fn get<T>(&self, idx: CFIndex) -> Option<&T> {
        (CFArrayGetValueAtIndex(self as *const _ as _, idx) as *const T).as_ref()
    }
}
/// toll-free bridging
impl<T: ObjcObject> AsRef<NSArray<T>> for CFArray {
    fn as_ref(&self) -> &NSArray<T> {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: ObjcObject> AsRef<CFArray> for NSArray<T> {
    fn as_ref(&self) -> &CFArray {
        unsafe { core::mem::transmute(self) }
    }
}

/// Manages associations of key-value pairs.
pub enum CFDictionary {}
/// A reference to an immutable dictionary object.
pub type CFDictionaryRef = *mut CFDictionary;
unsafe impl CoreObject for CFDictionary {}
/// toll-free bridging
impl<K: ObjcObject, V: ObjcObject> AsRef<NSDictionary<K, V>> for CFDictionary {
    fn as_ref(&self) -> &NSDictionary<K, V> {
        unsafe { ::std::mem::transmute(self) }
    }
}
impl<K: ObjcObject, V: ObjcObject> AsRef<CFDictionary> for NSDictionary<K, V> {
    fn as_ref(&self) -> &CFDictionary {
        unsafe { ::std::mem::transmute(self) }
    }
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
unsafe impl CoreObject for CFAttributedString {}
TollfreeBridge!(CFAttributedString = NSAttributedString);

/// A structure representing a range of sequential items in a container.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CFRange {
    location: CFIndex,
    length: CFIndex,
}
impl From<Range<u32>> for CFRange {
    fn from(r: Range<u32>) -> Self {
        CFRange {
            location: r.start as _,
            length: r.len() as _,
        }
    }
}
impl From<Range<CFIndex>> for CFRange {
    fn from(r: Range<CFIndex>) -> Self {
        CFRange {
            location: r.start,
            length: r.end - r.start,
        }
    }
}
impl From<RangeFrom<u32>> for CFRange {
    fn from(r: RangeFrom<u32>) -> Self {
        CFRange {
            location: r.start as _,
            length: CFIndex::max_value(),
        }
    }
}
impl From<RangeTo<u32>> for CFRange {
    fn from(r: RangeTo<u32>) -> Self {
        CFRange {
            location: 0,
            length: r.end as _,
        }
    }
}
impl From<RangeFull> for CFRange {
    fn from(_: RangeFull) -> Self {
        CFRange { location: 0, length: 0 }
    }
}

pub enum CFString {}
/// A reference to a CFString object.
pub type CFStringRef = *mut CFString;
unsafe impl CoreObject for CFString {}
TollfreeBridge!(CFString = NSString);

pub enum CFNumber {}
/// A reference to a CFNumber object.
pub type CFNumberRef = *mut CFNumber;
unsafe impl CoreObject for CFNumber {}
TollfreeBridge!(CFNumber = NSNumber);

pub enum CFData {}
/// A reference to a CFData object.
pub type CFDataRef = *mut CFData;
unsafe impl CoreObject for CFData {}
impl CFData {
    pub fn new(v: &[u8]) -> Option<CoreRetainedObject<Self>> {
        unsafe { CoreRetainedObject::retained_checked(CFDataCreate(std::ptr::null_mut(), v.as_ptr(), v.len() as _)) }
    }
}

pub enum CFAllocator {}
/// A reference to a CFAllocator object.
pub type CFAllocatorRef = *mut CFAllocator;
unsafe impl CoreObject for CFAllocator {}

#[link(name = "CoreFoundation", kind = "framework")]
extern "system" {
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);
    fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> *const c_void;
    fn CFDictionaryGetValue(dict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
}

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

/// Marker trait for describing CoreFoundation object types.
pub unsafe trait CoreObject {
    #[inline(always)]
    unsafe fn retain(ptr: *const Self) {
        CFRetain(ptr as _);
    }

    #[inline(always)]
    unsafe fn release(ptr: *const Self) {
        CFRelease(ptr as _);
    }
}

/// autorelease box for CoreFoundation objects
#[repr(transparent)]
pub struct CoreRetainedObject<T: CoreObject>(*const T);
impl<T: CoreObject> CoreRetainedObject<T> {
    pub const unsafe fn retained_unchecked(ptr: *const T) -> Self {
        Self(ptr)
    }

    #[inline(always)]
    pub(crate) unsafe fn retained(ptr: *const T) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        Some(Self::retained_unchecked(ptr))
    }

    pub const fn as_ptr(&self) -> *const T {
        self.0
    }
}
impl<T: CoreObject> Clone for CoreRetainedObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            T::retain(self.as_ptr());
        }

        Self(self.0)
    }
}
impl<T: CoreObject> Drop for CoreRetainedObject<T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            T::release(self.as_ptr());
        }
    }
}
impl<T: CoreObject> core::ops::Deref for CoreRetainedObject<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl<T: CoreObject> AsRef<T> for CoreRetainedObject<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
impl<T: CoreObject> core::borrow::Borrow<T> for CoreRetainedObject<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        unsafe { &*self.0 }
    }
}

/// autorelease smart pointer for mutable CoreFoundation objects
#[repr(transparent)]
pub struct CoreRetainedMutableObject<T: CoreObject>(core::ptr::NonNull<T>);
impl<T: CoreObject> CoreRetainedMutableObject<T> {
    pub const unsafe fn retained(ptr: core::ptr::NonNull<T>) -> Self {
        Self(ptr)
    }

    #[inline(always)]
    pub unsafe fn from_retained_ptr(ptr: *mut T) -> Option<Self> {
        Some(Self(core::ptr::NonNull::new(ptr)?))
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}
impl<T: CoreObject> Clone for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            T::retain(self.0.as_ptr());
        }

        Self(self.0)
    }
}
impl<T: CoreObject> Drop for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            T::release(self.0.as_ptr());
        }
    }
}
impl<T: CoreObject> AsRef<T> for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: CoreObject> AsMut<T> for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}
impl<T: CoreObject> core::ops::Deref for CoreRetainedMutableObject<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T: CoreObject> core::ops::DerefMut for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T: CoreObject> core::borrow::Borrow<T> for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: CoreObject> core::borrow::BorrowMut<T> for CoreRetainedMutableObject<T> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

/// Priority values used for kAXPriorityKey.
pub type CFIndex = c_long;
DefineCoreObject! {
    /// Manages static ordered collections of values.
    pub CFArray;
}
/// A reference to an immutable array object.
pub type CFArrayRef = *const CFArray;
impl CFArray {
    /// Returns the number of values currently in an array.
    #[inline(always)]
    pub fn len(&self) -> CFIndex {
        unsafe { CFArrayGetCount(self as *const _ as _) }
    }

    /// Retrieves a value at a given index.
    #[inline(always)]
    pub unsafe fn get<T>(&self, idx: CFIndex) -> Option<&T> {
        (CFArrayGetValueAtIndex(self as *const _ as _, idx) as *const T).as_ref()
    }

    /// toll-free bridging but no type-safety provided
    pub const unsafe fn as_nsarray_ref_unchecked<T: ObjcObject>(&self) -> &NSArray<T> {
        core::mem::transmute(self)
    }
}
/// toll-free bridging
impl<T: ObjcObject> AsRef<CFArray> for NSArray<T> {
    #[inline(always)]
    fn as_ref(&self) -> &CFArray {
        unsafe { core::mem::transmute(self) }
    }
}

DefineCoreObject! {
    /// Manages associations of key-value pairs.
    pub CFDictionary;
}
/// A reference to an immutable dictionary object.
pub type CFDictionaryRef = *const CFDictionary;
/// toll-free bridging
impl<K: ObjcObject, V: ObjcObject> AsRef<NSDictionary<K, V>> for CFDictionary {
    #[inline(always)]
    fn as_ref(&self) -> &NSDictionary<K, V> {
        unsafe { core::mem::transmute(self) }
    }
}
impl CFDictionary {
    /// Returns the value associated with a given key.
    #[inline(always)]
    pub unsafe fn get<K, T>(&self, key: &K) -> Option<&T> {
        (CFDictionaryGetValue(self as _, key as *const K as _) as *const T).as_ref()
    }

    /// toll-free bridging but no type-safety provided
    pub const unsafe fn as_nsdictionary_ref_unchecked<K: ObjcObject, V: ObjcObject>(&self) -> &NSDictionary<K, V> {
        core::mem::transmute(self)
    }
}

DefineCoreObject! {
    /// Manages character strings and associated sets of attributes.
    pub CFAttributedString;
}
/// A reference to a CFAttributedString object.
pub type CFAttributedStringRef = *const CFAttributedString;
TollfreeBridge!(CFAttributedString = NSAttributedString);

/// A structure representing a range of sequential items in a container.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}
impl From<Range<CFIndex>> for CFRange {
    #[inline(always)]
    fn from(r: Range<CFIndex>) -> Self {
        Self {
            location: r.start,
            length: r.end - r.start,
        }
    }
}
impl From<RangeFrom<CFIndex>> for CFRange {
    #[inline(always)]
    fn from(r: RangeFrom<CFIndex>) -> Self {
        Self {
            location: r.start,
            length: CFIndex::max_value(),
        }
    }
}
impl From<RangeTo<CFIndex>> for CFRange {
    #[inline(always)]
    fn from(r: RangeTo<CFIndex>) -> Self {
        Self {
            location: 0,
            length: r.end,
        }
    }
}
impl From<RangeFull> for CFRange {
    #[inline(always)]
    fn from(_: RangeFull) -> Self {
        Self { location: 0, length: 0 }
    }
}

DefineCoreObject! {
    pub CFString;
}
/// A reference to a CFString object.
pub type CFStringRef = *const CFString;
TollfreeBridge!(CFString = NSString);

DefineCoreObject! {
    pub CFNumber;
}
/// A reference to a CFNumber object.
pub type CFNumberRef = *mut CFNumber;
TollfreeBridge!(CFNumber = NSNumber);

DefineCoreObject! {
    pub CFData;
}
/// A reference to a CFData object.
pub type CFDataRef = *const CFData;
pub type CFMutableDataRef = *mut CFData;
impl CFData {
    #[inline(always)]
    pub fn new(v: &[u8]) -> Option<CoreRetainedObject<Self>> {
        unsafe { CoreRetainedObject::retained(CFDataCreate(std::ptr::null_mut(), v.as_ptr(), v.len() as _)) }
    }
}

DefineCoreObject! {
    pub CFAllocator;
}
/// A reference to a CFAllocator object.
pub type CFAllocatorRef = *mut CFAllocator;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "system" {
    unsafe fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    unsafe fn CFRelease(cf: CFTypeRef);
    unsafe fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    unsafe fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> *const c_void;
    unsafe fn CFDictionaryGetValue(dict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    unsafe fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
}

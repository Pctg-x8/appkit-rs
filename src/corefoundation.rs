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
    pub unsafe fn get<T: ExternalRced>(&self, idx: CFIndex) -> Option<ExternalRc<T>> {
        T::own_from(CFArrayGetValueAtIndex(self as *const _ as _, idx) as _)
    }
}
/// toll-free bridging
impl<T: ::ObjcObjectBase> AsRef<::NSArray<T>> for CFArray {
    fn as_ref(&self) -> &::NSArray<T> { unsafe { ::std::mem::transmute(self) } }
}
impl<T: ::ObjcObjectBase> AsRef<CFArray> for ::NSArray<T> {
    fn as_ref(&self) -> &CFArray { unsafe { ::std::mem::transmute(self) } }
}

#[link(name = "CoreFoundation", kind = "framework")] extern "system" {
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);
    fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> *const c_void;
}

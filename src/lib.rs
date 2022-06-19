//! CoreFoundation/Cocoa Framework

#[macro_use]
extern crate objc;
extern crate libc;
#[macro_use]
extern crate bitflags;

objc_ext::DefineObjcObjectWrapper!(pub NSObject);
impl NSObject {
    pub fn retain(&self) -> *mut Self {
        let p: *mut Object = unsafe { msg_send![&self.0, retain] };
        return p as *mut Self;
    }

    pub fn release(&self) {
        let _: () = unsafe { msg_send![&self.0, release] };
    }
}

#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;
#[cfg(not(target_pointer_width = "64"))]
pub type NSInteger = i32;
#[cfg(not(target_pointer_width = "64"))]
pub type NSUInteger = u32;

/// Declares toll-free bridge
macro_rules! TollfreeBridge {
    ($a: ty = $b: ty) => {
        impl AsRef<$a> for $b {
            fn as_ref(&self) -> &$a {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl AsMut<$a> for $b {
            fn as_mut(&mut self) -> &mut $a {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl AsRef<$b> for $a {
            fn as_ref(&self) -> &$b {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl AsMut<$b> for $a {
            fn as_mut(&mut self) -> &mut $b {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
}

pub type OSType = u32;
pub type OSStatus = i32;

mod corefoundation;
pub use corefoundation::*;
mod foundation;
pub use foundation::*;
mod appkit;
pub use appkit::*;
mod coregraphics;
pub use coregraphics::*;
mod corevideo;
pub use corevideo::*;
mod coreanimation;
pub use coreanimation::*;
mod coretext;
pub use coretext::*;
mod audiotoolbox;
pub use audiotoolbox::*;
use objc_ext::ObjcObject;

use std::borrow::{Borrow, BorrowMut, Cow, ToOwned};
use std::ops::{Deref, DerefMut};

pub struct ExternalRc<T> {
    ptr: *mut T,
    retain: unsafe extern "system" fn(*mut T) -> *mut T,
    release: unsafe extern "system" fn(*mut T),
}
impl<T> Deref for ExternalRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T> DerefMut for ExternalRc<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}
impl<T> Drop for ExternalRc<T> {
    fn drop(&mut self) {
        unsafe {
            (self.release)(self.ptr);
        }
    }
}
impl<T> Clone for ExternalRc<T> {
    fn clone(&self) -> Self {
        let new = unsafe { (self.retain)(self.ptr) };
        if new.is_null() {
            panic!("Retaining reference counted object");
        }
        return ExternalRc {
            ptr: self.ptr,
            retain: self.retain,
            release: self.release,
        };
    }
}
impl<T> ExternalRc<T> {
    pub(crate) unsafe fn with_fn(
        ptr: *mut T,
        retain: unsafe extern "system" fn(*mut T) -> *mut T,
        release: unsafe extern "system" fn(*mut T),
    ) -> Self {
        ExternalRc { ptr, retain, release }
    }
}
pub trait ExternalRced: Sized {
    unsafe fn own_from_unchecked(p: *mut Self) -> ExternalRc<Self>;
    unsafe fn own_from(p: *mut Self) -> Option<ExternalRc<Self>> {
        if p.is_null() {
            None
        } else {
            Some(Self::own_from_unchecked(p))
        }
    }
}

use objc::runtime::Object;
pub struct CocoaObject<T: ObjcObject>(*mut T);
impl<T: ObjcObject> Clone for CocoaObject<T> {
    fn clone(&self) -> Self {
        let p: *mut Object = unsafe { msg_send![self.id(), retain] };
        if p.is_null() {
            panic!("Retaining reference counted object");
        }
        return CocoaObject(self.0);
    }
}
impl<T: ObjcObject> Drop for CocoaObject<T> {
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![self.id(), release] };
    }
}
impl<T: ObjcObject> CocoaObject<T> {
    pub fn id(&self) -> *mut Object {
        self.0 as *mut _
    }
    pub fn into_id(self) -> *mut Object {
        let id = self.id();
        std::mem::forget(self);
        return id;
    }
    pub fn retain(obj: *mut T) -> Result<Self, ()> {
        unsafe { Self::from_id(msg_send![obj as *mut Object, retain]) }
    }
    /// Occurs null checking
    pub unsafe fn from_id(id: *mut Object) -> Result<Self, ()> {
        if id.is_null() {
            Err(())
        } else {
            Ok(Self::from_id_unchecked(id))
        }
    }
    pub unsafe fn from_id_unchecked(id: *mut Object) -> Self {
        CocoaObject(id as _)
    }
}
impl<T: ObjcObject> Deref for CocoaObject<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
impl<T: ObjcObject> DerefMut for CocoaObject<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}
impl<T: ObjcObject> Borrow<T> for CocoaObject<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}
impl<T: ObjcObject> BorrowMut<T> for CocoaObject<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}
impl ToOwned for NSString {
    type Owned = CocoaObject<Self>;
    fn to_owned(&self) -> Self::Owned {
        unsafe { std::mem::transmute::<_, &CocoaObject<Self>>(self).clone() }
    }
}
impl ToOwned for NSMenuItem {
    type Owned = CocoaObject<Self>;
    fn to_owned(&self) -> Self::Owned {
        unsafe { std::mem::transmute::<_, &CocoaObject<Self>>(self).clone() }
    }
}

/// Ref to NSString or Ref to str slice
pub trait CocoaString {
    fn to_nsstring(&self) -> Cow<CocoaObject<NSString>>;
}
impl CocoaString for CocoaObject<NSString> {
    fn to_nsstring(&self) -> Cow<CocoaObject<NSString>> {
        Cow::Borrowed(self)
    }
}
impl CocoaString for str {
    fn to_nsstring(&self) -> Cow<CocoaObject<NSString>> {
        Cow::Owned(NSString::from_str(self).unwrap())
    }
}
impl CocoaString for String {
    fn to_nsstring(&self) -> Cow<CocoaObject<NSString>> {
        Cow::Owned(NSString::from_str(self).unwrap())
    }
}

pub type UniChar = u16;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}
impl From<std::ops::Range<NSUInteger>> for NSRange {
    fn from(r: std::ops::Range<NSUInteger>) -> Self {
        NSRange {
            location: r.start,
            length: r.end - r.start,
        }
    }
}

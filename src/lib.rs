//! CoreFoundation/Cocoa Framework

#[macro_use]
extern crate objc;
extern crate libc;
#[macro_use]
extern crate bitflags;

// strictly defined ffi object: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
macro_rules! DefineOpaqueFFIObject {
    ($(#[$a: meta])* $v: vis struct $name: ident) => {
        #[repr(C)]
        $(#[$a])*
        $v struct $name([u8; 0], core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>);
    };
    { $(#[$a: meta])* $v: vis struct $name: ident; } => {
        #[repr(C)]
        $(#[$a])*
        $v struct $name([u8; 0], core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>);
    }
}

macro_rules! DefineCoreObject {
    ($(#[$a: meta])* $v: vis $name: ident) => {
        DefineOpaqueFFIObject!($(#[$a])* $v struct $name);
        unsafe impl CoreObject for $name {}
    };
    { $(#[$a: meta])* $v: vis $name: ident; } => {
        DefineOpaqueFFIObject!($(#[$a])* $v struct $name);
        unsafe impl $crate::corefoundation::CoreObject for $name {}
    };
}

objc_ext::DefineObjcObjectWrapper!(pub NSObject);
impl NSObject {
    pub fn retain(&self) -> *mut Self {
        let p: *mut Object = unsafe { msg_send![self.as_id(), retain] };

        p as *mut Self
    }

    pub fn release(&self) {
        let _: () = unsafe { msg_send![self.as_id(), release] };
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
    (mut $a: ty = $b: ty) => {
        impl AsRef<$a> for $b {
            #[inline(always)]
            fn as_ref(&self) -> &$a {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl AsMut<$a> for $b {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $a {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl AsRef<$b> for $a {
            #[inline(always)]
            fn as_ref(&self) -> &$b {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl AsMut<$b> for $a {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $b {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
    ($a: ty = $b: ty) => {
        impl AsRef<$a> for $b {
            #[inline(always)]
            fn as_ref(&self) -> &$a {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl AsRef<$b> for $a {
            #[inline(always)]
            fn as_ref(&self) -> &$b {
                unsafe { core::mem::transmute(self) }
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

use objc::runtime::Object;

pub struct CocoaObject<T: ObjcObject>(core::ptr::NonNull<T>);
unsafe impl<T: ObjcObject + Sync> Sync for CocoaObject<T> {}
unsafe impl<T: ObjcObject + Send> Send for CocoaObject<T> {}
impl<T: ObjcObject> Clone for CocoaObject<T> {
    fn clone(&self) -> Self {
        let p: *mut Object = unsafe { msg_send![self.id(), retain] };
        if p.is_null() {
            panic!("Retaining reference counted object");
        }

        CocoaObject(self.0)
    }
}
impl<T: ObjcObject> Drop for CocoaObject<T> {
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![self.id(), release] };
    }
}
impl<T: ObjcObject> CocoaObject<T> {
    pub fn id(&self) -> *mut Object {
        self.0.as_ptr() as _
    }
    pub fn into_id(self) -> *mut Object {
        let id = self.id();
        // no drop executes
        core::mem::forget(self);

        id
    }
    pub fn retain(obj: *mut T) -> Result<Self, ()> {
        unsafe { Self::from_id(msg_send![obj as *mut Object, retain]) }
    }
    /// Occurs null checking
    pub unsafe fn from_id(id: *mut Object) -> Result<Self, ()> {
        core::ptr::NonNull::new(id as _).map(Self).ok_or(())
    }
    pub unsafe fn from_id_unchecked(id: *mut Object) -> Self {
        CocoaObject(core::ptr::NonNull::new_unchecked(id as _))
    }
}
impl<T: ObjcObject> Deref for CocoaObject<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: ObjcObject> DerefMut for CocoaObject<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}
impl<T: ObjcObject> Borrow<T> for CocoaObject<T> {
    fn borrow(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: ObjcObject> BorrowMut<T> for CocoaObject<T> {
    fn borrow_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}
impl ToOwned for NSMenuItem {
    type Owned = CocoaObject<Self>;
    fn to_owned(&self) -> Self::Owned {
        unsafe { std::mem::transmute::<_, &CocoaObject<Self>>(self).clone() }
    }
}
unsafe impl<T: ObjcObject> ObjcObject for CocoaObject<T> {
    fn as_id(&self) -> &objc::runtime::Object {
        T::as_id(unsafe { self.0.as_ref() })
    }

    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        T::as_id_mut(unsafe { self.0.as_mut() })
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

const fn opt_pointer<T>(opt: Option<&T>) -> *const T {
    match opt {
        Some(r) => r as *const _,
        None => core::ptr::null(),
    }
}

const fn opt_pointer_mut<T>(opt: Option<&mut T>) -> *mut T {
    match opt {
        Some(r) => r as *mut _,
        None => core::ptr::null_mut(),
    }
}

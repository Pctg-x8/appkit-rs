//! CoreFoundation/Cocoa Framework

use objc::{msg_send, runtime::Object, sel, sel_impl};
use objc_ext::ObjcObject;

use std::borrow::ToOwned;

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
    #[inline(always)]
    pub fn retain(&self) -> *mut Self {
        let p: *mut Object = unsafe { msg_send![self, retain] };
        p as *mut Self
    }

    #[inline(always)]
    pub fn release(&self) {
        let _: () = unsafe { msg_send![self, release] };
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

/// A smart pointer for NSObject children
#[repr(transparent)]
pub struct CocoaObject<T: ObjcObject>(*const T);
unsafe impl<T: ObjcObject + Sync> Sync for CocoaObject<T> {}
unsafe impl<T: ObjcObject + Send> Send for CocoaObject<T> {}
impl<T: ObjcObject> Clone for CocoaObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        let _: *mut Object = unsafe { msg_send![self.id(), retain] };
        Self(self.0)
    }
}
impl<T: ObjcObject> Drop for CocoaObject<T> {
    #[inline(always)]
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![self.id(), release] };
    }
}
impl<T: ObjcObject> CocoaObject<T> {
    pub const fn id(&self) -> *const Object {
        self.0 as _
    }

    pub const fn into_id(self) -> *const Object {
        let id = self.id();
        // no drop executes
        core::mem::forget(self);

        id
    }

    pub const unsafe fn from_retained_ptr_unchecked(ptr: *const T) -> Self {
        Self(ptr)
    }

    pub const unsafe fn from_retained_id_unchecked(id: *const Object) -> Self {
        Self(id as _)
    }

    #[inline(always)]
    pub unsafe fn from_retained_ptr(ptr: *const T) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_retained_ptr_unchecked(ptr))
        }
    }

    #[inline(always)]
    pub unsafe fn from_retained_id(id: *const Object) -> Option<Self> {
        if id.is_null() {
            None
        } else {
            Some(Self::from_retained_id_unchecked(id))
        }
    }

    pub fn retain(obj: &T) -> Self
    where
        T: objc::Message,
    {
        let _: *mut Object = unsafe { msg_send![obj, retain] };
        Self(obj)
    }
}
impl<T: ObjcObject> core::ops::Deref for CocoaObject<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
impl<T: ObjcObject> core::borrow::Borrow<T> for CocoaObject<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        unsafe { &*self.0 }
    }
}

/// A smart pointer for NSObject children
#[repr(transparent)]
pub struct CocoaMutableObject<T: ObjcObject>(core::ptr::NonNull<T>);
unsafe impl<T: ObjcObject + Sync> Sync for CocoaMutableObject<T> {}
unsafe impl<T: ObjcObject + Send> Send for CocoaMutableObject<T> {}
impl<T: ObjcObject> Clone for CocoaMutableObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        let _: *mut Object = unsafe { msg_send![self.id(), retain] };
        Self(self.0)
    }
}
impl<T: ObjcObject> Drop for CocoaMutableObject<T> {
    #[inline(always)]
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![self.id(), release] };
    }
}
impl<T: ObjcObject> CocoaMutableObject<T> {
    pub const fn id(&self) -> *mut Object {
        self.0.as_ptr() as _
    }

    pub const fn into_id(self) -> *mut Object {
        let id = self.id();
        // no drop executed(moveout a pointer with its ownership)
        core::mem::forget(self);

        id
    }

    pub const fn from_retained_ptr_unchecked(ptr: core::ptr::NonNull<T>) -> Self {
        Self(ptr)
    }

    pub const unsafe fn from_retained_id_unchecked(id: core::ptr::NonNull<Object>) -> Self {
        Self(core::ptr::NonNull::new_unchecked(id.as_ptr() as _))
    }

    #[inline(always)]
    pub fn from_retained_ptr(id: *mut T) -> Option<Self> {
        Some(Self(core::ptr::NonNull::new(id)?))
    }

    #[inline(always)]
    pub fn from_retained_id(id: *mut Object) -> Option<Self> {
        Self::from_retained_ptr(id as _)
    }

    #[inline(always)]
    pub fn retain(ptr: &mut T) -> Self
    where
        T: objc::Message,
    {
        let _: *mut Object = unsafe { msg_send![ptr, retain] };
        Self(core::ptr::NonNull::from(ptr))
    }
}
impl<T: ObjcObject> core::ops::Deref for CocoaMutableObject<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T: ObjcObject> core::ops::DerefMut for CocoaMutableObject<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T: ObjcObject> core::borrow::Borrow<T> for CocoaMutableObject<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T: ObjcObject> core::borrow::BorrowMut<T> for CocoaMutableObject<T> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl ToOwned for NSMenuItem {
    type Owned = CocoaObject<Self>;

    #[inline(always)]
    fn to_owned(&self) -> Self::Owned {
        CocoaObject::retain(self)
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

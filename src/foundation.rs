//! Foundation APIs

use objc::runtime::*;
use objc_ext::ObjcObject;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::os::raw::*;
use std::ptr::null;

use crate::{CocoaObject, CocoaString, NSInteger, NSObject, NSUInteger};

// A static, plain-text Unicode string object.
objc_ext::DefineObjcObjectWrapper!(pub NSString : NSObject);
impl NSString {
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSString), alloc] };
        if p.is_null() {
            Err(())
        } else {
            Ok(p)
        }
    }

    pub fn empty() -> &'static Self {
        let p: *mut Object = unsafe { msg_send![class!(NSString), string] };

        unsafe {
            (p as *const Self)
                .as_ref()
                .expect("Nil returned from [NSString string]")
        }
    }

    pub fn from_str(s: &str) -> Result<CocoaObject<Self>, ()> {
        let bytes = s.as_bytes();
        unsafe {
            CocoaObject::from_id(msg_send![Self::alloc()?,
                initWithBytes: bytes.as_ptr() as *const c_void length: bytes.len() encoding: 4 as NSUInteger])
        }
    }

    pub fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(msg_send![self.as_id(), UTF8String]) }
    }

    pub fn to_str(&self) -> &str {
        self.to_cstr().to_str().unwrap()
    }
}
impl ToOwned for NSString {
    type Owned = CocoaObject<Self>;

    fn to_owned(&self) -> Self::Owned {
        unsafe { core::mem::transmute::<_, &CocoaObject<Self>>(self).clone() }
    }
}

// An object wrapper for primitive scalar numeric values.
objc_ext::DefineObjcObjectWrapper!(pub NSNumber : NSValue);
objc_ext::DefineObjcObjectWrapper!(pub NSValue : NSObject);
impl NSNumber {
    /// Creates and returns an NSNumber object containing a given value, treating it as a `float`.
    pub fn from_float(v: c_float) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSNumber), numberWithFloat: v]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an `unsigned int`.
    pub fn from_uint(v: c_uint) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSNumber), numberWithUnsignedInt: v]) }
    }
}

/// A static collection of objects associated with unique keys.
#[repr(C)]
pub struct NSDictionary<KeyType: ObjcObject, ObjectType: ObjcObject>(
    Object,
    PhantomData<(*mut KeyType, *mut ObjectType)>,
);
unsafe impl<K: ObjcObject, V: ObjcObject> ObjcObject for NSDictionary<K, V> {
    fn as_id(&self) -> &Object {
        &self.0
    }

    fn as_id_mut(&mut self) -> &mut Object {
        &mut self.0
    }
}
impl<K: ObjcObject, V: ObjcObject> core::ops::Deref for NSDictionary<K, V> {
    type Target = NSObject;

    fn deref(&self) -> &NSObject {
        unsafe { core::mem::transmute(self) }
    }
}
impl<K: ObjcObject, V: ObjcObject> core::ops::DerefMut for NSDictionary<K, V> {
    fn deref_mut(&mut self) -> &mut NSObject {
        unsafe { core::mem::transmute(self) }
    }
}

/// A dynamic collection of objects associated with unique keys.
#[repr(C)]
pub struct NSMutableDictionary<KeyType: NSCopying, ObjectType: ObjcObject>(
    Object,
    PhantomData<(*mut KeyType, *mut ObjectType)>,
);
unsafe impl<K: NSCopying, V: ObjcObject> ObjcObject for NSMutableDictionary<K, V> {
    fn as_id(&self) -> &Object {
        &self.0
    }

    fn as_id_mut(&mut self) -> &mut Object {
        &mut self.0
    }
}
impl<K: NSCopying, V: ObjcObject> std::ops::Deref for NSMutableDictionary<K, V> {
    type Target = NSDictionary<K, V>;

    fn deref(&self) -> &NSDictionary<K, V> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<K: NSCopying, V: ObjcObject> std::ops::DerefMut for NSMutableDictionary<K, V> {
    fn deref_mut(&mut self) -> &mut NSDictionary<K, V> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<KeyType: NSCopying, ObjectType: ObjcObject> NSMutableDictionary<KeyType, ObjectType> {
    /// Creates and returns a mutable dictionary, initially giving it enough allocated memory to
    /// hold a given number of entries.
    pub fn with_capacity(cap: NSUInteger) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSMutableDictionary), dictionaryWithCapacity: cap]) }
    }

    /// Creates a newly allocated mutable dictionary
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSMutableDictionary), dictionary]) }
    }

    /// Adds a given key-value pair to the dictionary.
    pub fn set(&mut self, key: &KeyType, object: &ObjectType) {
        let _: () = unsafe { msg_send![self.as_id_mut(), setObject: object.as_id() forKey: key.as_id()] };
    }

    /// Removes a given key and its associated value from the dictionary.
    pub fn remove(&mut self, key: &KeyType) {
        let _: () = unsafe { msg_send![self.as_id_mut(), removeObjectForKey: key.as_id()] };
    }

    /// Empties the dictionary of its entries.
    pub fn clear(&mut self) {
        let _: () = unsafe { msg_send![self.as_id_mut(), removeAllObjects] };
    }
}
impl<KeyType: ObjcObject, ObjectType: ObjcObject> NSDictionary<KeyType, ObjectType> {
    /// The number of entries in the dictionary.
    pub fn len(&self) -> NSUInteger {
        unsafe { msg_send![self.as_id(), count] }
    }

    /// Returns the value associated with a given key.
    pub fn get(&self, keytype: &KeyType) -> &ObjectType {
        let p: *mut Object = unsafe { msg_send![self.as_id(), objectForKey: keytype.as_id()] };
        unsafe { (p as *const ObjectType).as_ref().unwrap() }
    }
}

/// A static ordered collection of objects.
#[repr(C)]
pub struct NSArray<ObjectType: ObjcObject>(Object, PhantomData<*mut ObjectType>);
unsafe impl<O: ObjcObject> ObjcObject for NSArray<O> {
    fn as_id(&self) -> &objc::runtime::Object {
        &self.0
    }

    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        &mut self.0
    }
}
impl<O: ObjcObject> std::ops::Deref for NSArray<O> {
    type Target = NSObject;
    fn deref(&self) -> &NSObject {
        unsafe { std::mem::transmute(self) }
    }
}
impl<O: ObjcObject> std::ops::DerefMut for NSArray<O> {
    fn deref_mut(&mut self) -> &mut NSObject {
        unsafe { std::mem::transmute(self) }
    }
}

/// A dynamic ordered collection of objects.
#[repr(C)]
pub struct NSMutableArray<ObjectType: ObjcObject>(Object, PhantomData<*mut ObjectType>);
unsafe impl<O: ObjcObject> ObjcObject for NSMutableArray<O> {
    fn as_id(&self) -> &objc::runtime::Object {
        &self.0
    }

    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        &mut self.0
    }
}
impl<O: ObjcObject> std::ops::Deref for NSMutableArray<O> {
    type Target = NSArray<O>;
    fn deref(&self) -> &NSArray<O> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<O: ObjcObject> std::ops::DerefMut for NSMutableArray<O> {
    fn deref_mut(&mut self) -> &mut NSArray<O> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<ObjectType: ObjcObject> NSMutableArray<ObjectType> {
    /// Creates a newly allocated array.
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSMutableArray), array]) }
    }

    /// Creates and returns an `NSMutableArray` object with enough allocated memory to initially hold a given number of objects.
    pub fn with_capacity(cap: NSUInteger) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSMutableArray), arrayWithCapacity: cap]) }
    }

    /// Inserts a given object at the end of the array.
    pub fn push(&mut self, object: &ObjectType) {
        let _: () = unsafe { msg_send![self.as_id_mut(), addObject: object.as_id()] };
    }

    /// Inserts a given object into the array's contents at a given index.
    pub fn insert(&mut self, index: NSUInteger, object: &ObjectType) {
        let _: () = unsafe { msg_send![self.as_id_mut(), insertObject: object.as_id() atIndex: index] };
    }

    /// Empties the array of all its elements.
    pub fn clear(&mut self) {
        let _: () = unsafe { msg_send![self.as_id_mut(), removeAllObjects] };
    }
}
impl<ObjectType: ObjcObject> NSArray<ObjectType> {
    /// The number of objects in the array.
    pub fn len(&self) -> NSUInteger {
        unsafe { msg_send![self.as_id(), count] }
    }

    /// Returns the object located at the specified index.
    pub fn get(&self, index: NSUInteger) -> &ObjectType {
        let p: *mut Object = unsafe { msg_send![self.as_id(), objectAtIndex: index] };
        unsafe { (p as *const ObjectType).as_ref().unwrap() }
    }
}

// A representation of the code and resources stored in a bundle directory on disk.
objc_ext::DefineObjcObjectWrapper!(pub NSBundle : NSObject);
impl NSBundle {
    /// Returns the bundle object that contains the current executable.
    pub fn main() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSBundle), mainBundle] };
        unsafe { (p as *const NSBundle).as_ref().ok_or(()) }
    }

    /// Returns the value associated with the specified key in the receiver's information property list.
    pub fn object_for_info_dictionary_key<V>(&self, key: &(impl CocoaString + ?Sized)) -> Option<&V> {
        let k = key.to_nsstring();
        unsafe {
            let p: *mut Object = msg_send![self.as_id(), objectForInfoDictionaryKey: k.as_id()];
            (p as *const V).as_ref()
        }
    }
}

// A collection of information about the current process.
objc_ext::DefineObjcObjectWrapper!(pub NSProcessInfo : NSObject);
impl NSProcessInfo {
    /// Returns the process information agent for the process.
    pub fn current() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSProcessInfo), processInfo] };
        unsafe { (p as *const NSProcessInfo).as_ref().ok_or(()) }
    }

    /// The name of the process.
    pub fn name(&self) -> &NSString {
        unsafe {
            let p: *mut Object = msg_send![self.as_id(), processName];
            &*(p as *const NSString)
        }
    }
}

pub type NSAttributedStringKey = NSString;
// A string that has associated attributes for portions of its text.
objc_ext::DefineObjcObjectWrapper!(pub NSAttributedString : NSObject);
unsafe impl NSCopying for NSAttributedString {}
impl NSAttributedString {
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSAttributedString), alloc] };
        if p.is_null() {
            Err(())
        } else {
            Ok(p)
        }
    }

    pub fn new(
        s: &NSString,
        attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
    ) -> Result<CocoaObject<Self>, ()> {
        let p: *mut Object = unsafe {
            if let Some(a) = attrs {
                msg_send![Self::alloc()?, initWithString: s.as_id() attributes: a.as_id()]
            } else {
                msg_send![Self::alloc()?, initWithString: s.as_id()]
            }
        };
        unsafe { CocoaObject::from_id(p) }
    }
}

pub type NSErrorDomain = NSString;
pub type NSErrorUserInfoKey = NSString;

objc_ext::DefineObjcObjectWrapper!(pub NSError : NSObject);
unsafe impl NSCopying for NSError {}
impl NSError {
    pub fn code(&self) -> NSInteger {
        unsafe { msg_send![self.as_id(), code] }
    }
    pub fn domain(&self) -> Result<CocoaObject<NSErrorDomain>, ()> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), domain]) }
    }
    pub fn userinfo(&self) -> Result<CocoaObject<NSDictionary<NSErrorUserInfoKey, Object>>, ()> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), userinfo]) }
    }

    pub fn localized_description(&self) -> Result<CocoaObject<NSString>, ()> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), localizedDescription]) }
    }
    pub fn localized_recovery_options(&self) -> Option<CocoaObject<NSArray<NSString>>> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), localizedRecoveryOptions]).ok() }
    }
    pub fn localized_recovery_suggestion(&self) -> Option<CocoaObject<NSString>> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), localizedRecoverySuggestion]).ok() }
    }
    pub fn localized_failure_reason(&self) -> Option<CocoaObject<NSString>> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), localizedFailureReason]).ok() }
    }
}

/// A protocol that objects adopt to provide functional copies of themselves.
pub unsafe trait NSCopying: ObjcObject + Sized {
    /// Returns a new instance that's a copy of the receiver.
    /// This method will call `copyWithZone` with nil.
    fn copy(&self) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![self.as_id(), copyWithZone: null::<Object>()]) }
    }
}
unsafe impl<'a, T: 'a + ?Sized> NSCopying for &'a mut T
where
    T: NSCopying,
    &'a mut T: ObjcObject,
{
}
unsafe impl<O: ObjcObject> NSCopying for NSArray<O> {}
unsafe impl<K: ObjcObject, O: ObjcObject> NSCopying for NSDictionary<K, O> {}
unsafe impl NSCopying for NSString {}

//! Foundation APIs

use objc::runtime::*;
use {CocoaObject, ObjcObjectBase, NSObject};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::marker::PhantomData;
use std::ptr::null;

/// A static, plain-text Unicode string object.
#[derive(ObjcObjectBase)]
pub struct NSString(Object); DeclareClassDerivative!(NSString : NSObject);
impl NSString {
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSString").unwrap(), alloc] };
        if p.is_null() { Err(()) } else { Ok(p) }
    }
    pub fn empty() -> &'static Self {
        let p: *mut Object = unsafe { msg_send![Class::get("NSString").unwrap(), string] };
        return unsafe { (p as *const Self).as_ref().expect("Nil returned from [NSString string]") };
    }
    pub fn from_str(s: &str) -> Result<CocoaObject<Self>, ()> {
        let bytes = s.as_bytes();
        unsafe {
            CocoaObject::from_id(msg_send![Self::alloc()?,
                initWithBytes: bytes.as_ptr() as *const c_void length: bytes.len() encoding: 4 as ::NSUInteger])
        }
    }
    pub fn to_str(&self) -> &str { unsafe { CStr::from_ptr(msg_send![&self.0, UTF8String]).to_str().unwrap() } }
}

/// A static collection of objects associated with unique keys.
#[derive(ObjcObjectBase)]
pub struct NSDictionary<KeyType: ObjcObjectBase, ObjectType: ObjcObjectBase>
    (Object, PhantomData<(*mut KeyType, *mut ObjectType)>);
DeclareClassDerivative!(NSDictionary<K: NSCopying, O: ObjcObjectBase> : NSObject);
/// A dynamic collection of objects associated with unique keys.
#[derive(ObjcObjectBase)]
pub struct NSMutableDictionary<KeyType: NSCopying, ObjectType: ObjcObjectBase>
    (Object, PhantomData<(*mut KeyType, *mut ObjectType)>);
DeclareClassDerivative!(NSMutableDictionary<K: NSCopying, O: ObjcObjectBase> : NSDictionary<K, O>);
impl<KeyType: NSCopying, ObjectType: ObjcObjectBase> NSMutableDictionary<KeyType, ObjectType> {
    /// Creates and returns a mutable dictionary, initially giving it enough allocated memory to
    /// hold a given number of entries.
    pub fn with_capacity(cap: ::NSUInteger) -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_id(msg_send![Class::get("NSMutableDictionary").unwrap(), dictionaryWithCapacity: cap])
        }
    }
    /// Creates a newly allocated mutable dictionary
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_id(msg_send![Class::get("NSMutableDictionary").unwrap(), new])
        }
    }
    /// Adds a given key-value pair to the dictionary.
    pub fn set(&mut self, key: &KeyType, object: &ObjectType) {
        let _: () = unsafe { msg_send![self.objid_mut(), setObject: object.objid() forKey: key.objid()] };
    }
    /// Removes a given key and its associated value from the dictionary.
    pub fn remove(&mut self, key: &KeyType) {
        let _: () = unsafe { msg_send![self.objid_mut(), removeObjectForKey: key.objid()] };
    }
    /// Empties the dictionary of its entries.
    pub fn clear(&mut self) { let _: () = unsafe { msg_send![self.objid_mut(), removeAllObjects] }; }
}
impl<KeyType: ObjcObjectBase, ObjectType: ObjcObjectBase> NSDictionary<KeyType, ObjectType> {
    /// The number of entries in the dictionary.
    pub fn len(&self) -> ::NSUInteger { unsafe { msg_send![self.objid(), count] } }
    /// Returns the value associated with a given key.
    pub fn get(&self, keytype: &KeyType) -> &ObjectType {
        let p: *mut Object = unsafe { msg_send![self.objid(), objectForKey: keytype.objid()] };
        unsafe { (p as *const ObjectType).as_ref().unwrap() }
    }
}

/// A static ordered collection of objects.
#[derive(ObjcObjectBase)] pub struct NSArray<ObjectType: ObjcObjectBase>(Object, PhantomData<*mut ObjectType>);
DeclareClassDerivative!(NSArray<O: ObjcObjectBase> : NSObject);
/// A dynamic ordered collection of objects.
#[derive(ObjcObjectBase)]
pub struct NSMutableArray<ObjectType: ObjcObjectBase>(Object, PhantomData<*mut ObjectType>);
DeclareClassDerivative!(NSMutableArray<ObjectType: ObjcObjectBase> : NSArray<ObjectType>);
impl<ObjectType: ObjcObjectBase> NSMutableArray<ObjectType> {
    /// Creates a newly allocated array.
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![Class::get("NSMutableArray").unwrap(), new]) }
    }
    /// Creates and returns an `NSMutableArray` object with enough allocated memory to initially hold a given number of objects.
    pub fn with_capacity(cap: ::NSUInteger) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![Class::get("NSMutableArray").unwrap(), arrayWithCapacity: cap]) }
    }

    /// Inserts a given object at the end of the array.
    pub fn push(&mut self, object: &ObjectType) {
        let _: () = unsafe { msg_send![self.objid_mut(), addObject: object.objid()] };
    }
    /// Inserts a given object into the array's contents at a given index.
    pub fn insert(&mut self, index: ::NSUInteger, object: &ObjectType) {
        let _: () = unsafe { msg_send![self.objid_mut(), insertObject: object.objid() atIndex: index] };
    }
    /// Empties the array of all its elements.
    pub fn clear(&mut self) { let _: () = unsafe { msg_send![self.objid_mut(), removeAllObjects] }; }
}
impl<ObjectType: ObjcObjectBase> NSArray<ObjectType> {
    /// The number of objects in the array.
    pub fn len(&self) -> ::NSUInteger { unsafe { msg_send![self.objid(), count] } }
    /// Returns the object located at the specified index.
    pub fn get(&self, index: ::NSUInteger) -> &ObjectType {
        let p: *mut Object = unsafe { msg_send![self.objid(), objectAtIndex: index] };
        unsafe { (p as *const ObjectType).as_ref().unwrap() }
    }
}

/// A representation of the code and resources stored in a bundle directory on disk.
#[derive(ObjcObjectBase)] pub struct NSBundle(Object); DeclareClassDerivative!(NSBundle : NSObject);
impl NSBundle {
    /// Returns the bundle object that contains the current executable.
    pub fn main() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSBundle").unwrap(), mainBundle] };
        unsafe { (p as *const NSBundle).as_ref().ok_or(()) }
    }
    /// Returns the value associated with the specified key in the receiver's information property list.
    pub fn object_for_info_dictionary_key<K: ::CocoaString + ?Sized, V>(&self, key: &K) -> Option<&V> {
        let k = key.to_nsstring();
        unsafe {
            let p: *mut Object = msg_send![self.objid(), objectForInfoDictionaryKey: k.objid()];
            (p as *const V).as_ref()
        }
    }
}

/// A collection of information about the current process.
#[derive(ObjcObjectBase)] pub struct NSProcessInfo(Object); DeclareClassDerivative!(NSProcessInfo : NSObject);
impl NSProcessInfo {
    /// Returns the process information agent for the process.
    pub fn current() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSProcessInfo").unwrap(), processInfo] };
        unsafe { (p as *const NSProcessInfo).as_ref().ok_or(()) }
    }
    /// The name of the process.
    pub fn name(&self) -> &::NSString {
        unsafe { let p: *mut Object = msg_send![self.objid(), processName]; &*(p as *const ::NSString) }
    }
}

/// A protocol that objects adopt to provide functional copies of themselves.
pub unsafe trait NSCopying : ObjcObjectBase + Sized {
    /// Returns a new instance that's a copy of the receiver.
    /// This method will call `copyWithZone` with nil.
    fn copy(&self) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![self.objid(), copyWithZone: null::<Object>()]) }
    }
}
unsafe impl<O: ObjcObjectBase> NSCopying for NSArray<O> {}
unsafe impl<K: ObjcObjectBase, O: ObjcObjectBase> NSCopying for NSDictionary<K, O> {}
unsafe impl NSCopying for NSString {}

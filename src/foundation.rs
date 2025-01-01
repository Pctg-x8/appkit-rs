//! Foundation APIs

use objc::{class, msg_send, runtime::*, sel, sel_impl};
use objc_ext::ObjcObject;
use std::ffi::CStr;
use std::marker::PhantomData;

use crate::{CocoaMutableObject, CocoaObject, NSInteger, NSObject, NSUInteger};

objc_ext::DefineObjcObjectWrapper! {
    /// A static, plain-text Unicode string object.
    pub NSString : NSObject;
}
unsafe impl NSCopying for NSString {}
impl NSString {
    #[inline(always)]
    unsafe fn alloc() -> *mut Self {
        msg_send![class!(NSString), alloc]
    }

    /// Returns an empty string.
    #[inline(always)]
    pub fn empty() -> &'static Self {
        let p: *mut Object = unsafe { msg_send![class!(NSString), string] };
        unsafe { &*(p as *const Self) }
    }

    #[inline]
    pub fn from_str(s: &str) -> Result<CocoaMutableObject<Self>, ()> {
        let bytes = s.as_bytes();

        unsafe {
            CocoaMutableObject::from_retained_id(msg_send![Self::alloc(), initWithBytes: bytes.as_ptr() as *const core::ffi::c_void length: bytes.len() encoding: 4 as NSUInteger]).ok_or(())
        }
    }

    #[inline(always)]
    pub fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(msg_send![self, UTF8String]) }
    }

    #[inline]
    pub fn to_str(&self) -> &str {
        self.to_cstr().to_str().unwrap()
    }
}

objc_ext::DefineObjcObjectWrapper! {
    pub NSValue : NSObject;
}

objc_ext::DefineObjcObjectWrapper! {
    /// An object wrapper for primitive scalar numeric values.
    pub NSNumber : NSValue;
}
impl NSNumber {
    /// Creates and returns an NSNumber object containing a given value, treating it as a `float`.
    #[inline(always)]
    pub fn from_float(v: core::ffi::c_float) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_retained_id(msg_send![class!(NSNumber), numberWithFloat: v]).ok_or(()) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an `unsigned int`.
    #[inline(always)]
    pub fn from_uint(v: core::ffi::c_uint) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_retained_id(msg_send![class!(NSNumber), numberWithUnsignedInt: v]).ok_or(()) }
    }
}

/// A static collection of objects associated with unique keys.
#[repr(C)]
pub struct NSDictionary<KeyType: ObjcObject, ObjectType: ObjcObject>(
    Object,
    PhantomData<(*mut KeyType, *mut ObjectType)>,
);
unsafe impl<K: ObjcObject, V: ObjcObject> ObjcObject for NSDictionary<K, V> {
    #[inline(always)]
    fn as_id(&self) -> &Object {
        &self.0
    }

    #[inline(always)]
    fn as_id_mut(&mut self) -> &mut Object {
        &mut self.0
    }
}
unsafe impl<K: ObjcObject, V: ObjcObject> objc::Message for NSDictionary<K, V> {
    #[inline(always)]
    unsafe fn send_message<A, R>(&self, sel: Sel, args: A) -> Result<R, objc::MessageError>
    where
        Self: Sized,
        A: objc::MessageArguments,
        R: std::any::Any,
    {
        self.0.send_message::<A, R>(sel, args)
    }

    #[inline(always)]
    fn verify_message<A, R>(&self, sel: Sel) -> Result<(), objc::MessageError>
    where
        Self: Sized,
        A: objc::EncodeArguments,
        R: objc::Encode,
    {
        self.0.verify_message::<A, R>(sel)
    }
}
impl<K: ObjcObject, V: ObjcObject> core::ops::Deref for NSDictionary<K, V> {
    type Target = NSObject;

    #[inline(always)]
    fn deref(&self) -> &NSObject {
        unsafe { core::mem::transmute(self) }
    }
}
unsafe impl<K: ObjcObject, O: ObjcObject> NSCopying for NSDictionary<K, O> {}

/// A dynamic collection of objects associated with unique keys.
#[repr(C)]
pub struct NSMutableDictionary<KeyType: NSCopying, ObjectType: ObjcObject>(
    Object,
    PhantomData<(*mut KeyType, *mut ObjectType)>,
);
unsafe impl<K: NSCopying, V: ObjcObject> ObjcObject for NSMutableDictionary<K, V> {
    #[inline(always)]
    fn as_id(&self) -> &Object {
        &self.0
    }

    #[inline(always)]
    fn as_id_mut(&mut self) -> &mut Object {
        &mut self.0
    }
}
unsafe impl<K: NSCopying, V: ObjcObject> objc::Message for NSMutableDictionary<K, V> {
    #[inline(always)]
    unsafe fn send_message<A, R>(&self, sel: Sel, args: A) -> Result<R, objc::MessageError>
    where
        Self: Sized,
        A: objc::MessageArguments,
        R: std::any::Any,
    {
        self.0.send_message::<A, R>(sel, args)
    }

    #[inline(always)]
    fn verify_message<A, R>(&self, sel: Sel) -> Result<(), objc::MessageError>
    where
        Self: Sized,
        A: objc::EncodeArguments,
        R: objc::Encode,
    {
        self.0.verify_message::<A, R>(sel)
    }
}
impl<K: NSCopying, V: ObjcObject> std::ops::Deref for NSMutableDictionary<K, V> {
    type Target = NSDictionary<K, V>;

    #[inline(always)]
    fn deref(&self) -> &NSDictionary<K, V> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<K: NSCopying, V: ObjcObject> std::ops::DerefMut for NSMutableDictionary<K, V> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut NSDictionary<K, V> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<KeyType: NSCopying, ObjectType: ObjcObject> NSMutableDictionary<KeyType, ObjectType> {
    /// Creates and returns a mutable dictionary, initially giving it enough allocated memory to
    /// hold a given number of entries.
    #[inline(always)]
    pub fn with_capacity(cap: NSUInteger) -> Result<CocoaMutableObject<Self>, ()> {
        unsafe {
            CocoaMutableObject::from_retained_id(msg_send![class!(NSMutableDictionary), dictionaryWithCapacity: cap])
                .ok_or(())
        }
    }

    /// Creates a newly allocated mutable dictionary
    #[inline(always)]
    pub fn new() -> Result<CocoaMutableObject<Self>, ()> {
        unsafe { CocoaMutableObject::from_retained_id(msg_send![class!(NSMutableDictionary), dictionary]).ok_or(()) }
    }

    /// Adds a given key-value pair to the dictionary.
    #[inline(always)]
    pub fn set(&mut self, key: &KeyType, object: &ObjectType) {
        let _: () = unsafe { msg_send![self, setObject: object forKey: key] };
    }

    /// Removes a given key and its associated value from the dictionary.
    #[inline(always)]
    pub fn remove(&mut self, key: &KeyType) {
        let _: () = unsafe { msg_send![self, removeObjectForKey: key] };
    }

    /// Empties the dictionary of its entries.
    #[inline(always)]
    pub fn clear(&mut self) {
        let _: () = unsafe { msg_send![self, removeAllObjects] };
    }
}
impl<KeyType: ObjcObject, ObjectType: ObjcObject> NSDictionary<KeyType, ObjectType> {
    /// The number of entries in the dictionary.
    #[inline(always)]
    pub fn len(&self) -> NSUInteger {
        unsafe { msg_send![self, count] }
    }

    /// Returns the value associated with a given key.
    #[inline(always)]
    pub fn get(&self, key: &KeyType) -> &ObjectType {
        let p: *mut Object = unsafe { msg_send![self, objectForKey: key] };
        unsafe { &*(p as *const ObjectType) }
    }
}

/// A static ordered collection of objects.
#[repr(C)]
pub struct NSArray<ObjectType: ObjcObject>(Object, PhantomData<*mut ObjectType>);
unsafe impl<O: ObjcObject> ObjcObject for NSArray<O> {
    #[inline(always)]
    fn as_id(&self) -> &objc::runtime::Object {
        &self.0
    }

    #[inline(always)]
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        &mut self.0
    }
}
unsafe impl<O: ObjcObject> objc::Message for NSArray<O> {
    #[inline(always)]
    unsafe fn send_message<A, R>(&self, sel: Sel, args: A) -> Result<R, objc::MessageError>
    where
        Self: Sized,
        A: objc::MessageArguments,
        R: std::any::Any,
    {
        self.0.send_message::<A, R>(sel, args)
    }

    #[inline(always)]
    fn verify_message<A, R>(&self, sel: Sel) -> Result<(), objc::MessageError>
    where
        Self: Sized,
        A: objc::EncodeArguments,
        R: objc::Encode,
    {
        self.0.verify_message::<A, R>(sel)
    }
}
impl<O: ObjcObject> std::ops::Deref for NSArray<O> {
    type Target = NSObject;

    #[inline(always)]
    fn deref(&self) -> &NSObject {
        unsafe { std::mem::transmute(self) }
    }
}
unsafe impl<O: ObjcObject> NSCopying for NSArray<O> {}

/// A dynamic ordered collection of objects.
#[repr(C)]
pub struct NSMutableArray<ObjectType: ObjcObject>(Object, PhantomData<*mut ObjectType>);
unsafe impl<O: ObjcObject> ObjcObject for NSMutableArray<O> {
    #[inline(always)]
    fn as_id(&self) -> &objc::runtime::Object {
        &self.0
    }

    #[inline(always)]
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        &mut self.0
    }
}
unsafe impl<O: ObjcObject> objc::Message for NSMutableArray<O> {
    #[inline(always)]
    unsafe fn send_message<A, R>(&self, sel: Sel, args: A) -> Result<R, objc::MessageError>
    where
        Self: Sized,
        A: objc::MessageArguments,
        R: std::any::Any,
    {
        self.0.send_message::<A, R>(sel, args)
    }

    #[inline(always)]
    fn verify_message<A, R>(&self, sel: Sel) -> Result<(), objc::MessageError>
    where
        Self: Sized,
        A: objc::EncodeArguments,
        R: objc::Encode,
    {
        self.0.verify_message::<A, R>(sel)
    }
}
impl<O: ObjcObject> std::ops::Deref for NSMutableArray<O> {
    type Target = NSArray<O>;

    #[inline(always)]
    fn deref(&self) -> &NSArray<O> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<O: ObjcObject> std::ops::DerefMut for NSMutableArray<O> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut NSArray<O> {
        unsafe { std::mem::transmute(self) }
    }
}
impl<ObjectType: ObjcObject> NSMutableArray<ObjectType> {
    /// Creates a newly allocated array.
    #[inline(always)]
    pub fn new() -> Result<CocoaMutableObject<Self>, ()> {
        unsafe { CocoaMutableObject::from_retained_id(msg_send![class!(NSMutableArray), array]).ok_or(()) }
    }

    /// Creates and returns an `NSMutableArray` object with enough allocated memory to initially hold a given number of objects.
    #[inline(always)]
    pub fn with_capacity(cap: NSUInteger) -> Result<CocoaMutableObject<Self>, ()> {
        unsafe {
            CocoaMutableObject::from_retained_id(msg_send![class!(NSMutableArray), arrayWithCapacity: cap]).ok_or(())
        }
    }

    /// Inserts a given object at the end of the array.
    #[inline(always)]
    pub fn push(&mut self, object: &ObjectType) {
        let _: () = unsafe { msg_send![self, addObject: object] };
    }

    /// Inserts a given object into the array's contents at a given index.
    #[inline(always)]
    pub fn insert(&mut self, index: NSUInteger, object: &ObjectType) {
        let _: () = unsafe { msg_send![self, insertObject: object atIndex: index] };
    }

    /// Empties the array of all its elements.
    #[inline(always)]
    pub fn clear(&mut self) {
        let _: () = unsafe { msg_send![self, removeAllObjects] };
    }
}
impl<ObjectType: ObjcObject> NSArray<ObjectType> {
    /// The number of objects in the array.
    #[inline(always)]
    pub fn len(&self) -> NSUInteger {
        unsafe { msg_send![self, count] }
    }

    /// Returns the object located at the specified index.
    #[inline(always)]
    pub fn get(&self, index: NSUInteger) -> &ObjectType {
        let p: *mut Object = unsafe { msg_send![self, objectAtIndex: index] };
        unsafe { &*(p as *const ObjectType) }
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// A representation of the code and resources stored in a bundle directory on disk.
    pub NSBundle : NSObject;
}
impl NSBundle {
    /// Returns the bundle object that contains the current executable.
    #[inline(always)]
    pub fn main() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSBundle), mainBundle] };
        unsafe { (p as *const NSBundle).as_ref().ok_or(()) }
    }

    /// Returns the value associated with the specified key in the receiver's information property list.
    #[inline(always)]
    pub unsafe fn object_for_info_dictionary_key<V>(&self, key: &NSString) -> Option<&V> {
        let p: *mut Object = msg_send![self, objectForInfoDictionaryKey: key];
        (p as *const V).as_ref()
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// A collection of information about the current process.
    pub NSProcessInfo : NSObject;
}
impl NSProcessInfo {
    /// Returns the process information agent for the process.
    #[inline(always)]
    pub fn current() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSProcessInfo), processInfo] };
        unsafe { (p as *const NSProcessInfo).as_ref().ok_or(()) }
    }

    /// The name of the process.
    #[inline(always)]
    pub fn name(&self) -> &NSString {
        unsafe {
            let p: *mut Object = msg_send![self, processName];
            &*(p as *const NSString)
        }
    }
}

pub type NSAttributedStringKey = NSString;
objc_ext::DefineObjcObjectWrapper! {
    /// A string that has associated attributes for portions of its text.
    pub NSAttributedString : NSObject;
}
unsafe impl NSCopying for NSAttributedString {}
impl NSAttributedString {
    #[inline(always)]
    fn alloc() -> *mut Object {
        unsafe { msg_send![class!(NSAttributedString), alloc] }
    }

    #[inline]
    pub fn new(
        s: &NSString,
        attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
    ) -> Result<CocoaObject<Self>, ()> {
        let p: *mut Object = unsafe {
            if let Some(a) = attrs {
                msg_send![Self::alloc(), initWithString: s attributes: a]
            } else {
                msg_send![Self::alloc(), initWithString: s]
            }
        };

        unsafe { CocoaObject::from_retained_id(p).ok_or(()) }
    }
}

pub type NSErrorDomain = NSString;
pub type NSErrorUserInfoKey = NSString;

objc_ext::DefineObjcObjectWrapper!(pub NSError : NSObject);
unsafe impl NSCopying for NSError {}
impl NSError {
    #[inline(always)]
    pub fn code(&self) -> NSInteger {
        unsafe { msg_send![self, code] }
    }

    #[inline(always)]
    pub fn domain(&self) -> Result<CocoaObject<NSErrorDomain>, ()> {
        unsafe { CocoaObject::from_retained_id(msg_send![self, domain]).ok_or(()) }
    }

    #[inline(always)]
    pub fn userinfo(&self) -> Result<CocoaObject<NSDictionary<NSErrorUserInfoKey, Object>>, ()> {
        unsafe { CocoaObject::from_retained_id(msg_send![self, userinfo]).ok_or(()) }
    }

    #[inline(always)]
    pub fn localized_description(&self) -> Result<CocoaObject<NSString>, ()> {
        unsafe { CocoaObject::from_retained_id(msg_send![self, localizedDescription]).ok_or(()) }
    }

    #[inline(always)]
    pub fn localized_recovery_options(&self) -> Option<CocoaObject<NSArray<NSString>>> {
        unsafe { CocoaObject::from_retained_id(msg_send![self, localizedRecoveryOptions]) }
    }

    #[inline(always)]
    pub fn localized_recovery_suggestion(&self) -> Option<CocoaObject<NSString>> {
        unsafe { CocoaObject::from_retained_id(msg_send![self, localizedRecoverySuggestion]) }
    }

    #[inline(always)]
    pub fn localized_failure_reason(&self) -> Option<CocoaObject<NSString>> {
        unsafe { CocoaObject::from_retained_id(msg_send![self, localizedFailureReason]) }
    }
}

/// A protocol that objects adopt to provide functional copies of themselves.
pub unsafe trait NSCopying: ObjcObject + Sized {
    /// Returns a new instance that's a copy of the receiver.
    /// This method will call `copyWithZone` with nil.
    #[inline(always)]
    fn copy(&self) -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_retained_id(msg_send![self.as_id(), copyWithZone: core::ptr::null::<Object>()]).ok_or(())
        }
    }
}
unsafe impl<'a, T: 'a + ?Sized> NSCopying for &'a mut T
where
    T: NSCopying,
    &'a mut T: ObjcObject,
{
}

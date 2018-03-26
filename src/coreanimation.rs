//! Core Animation

use objc::runtime::*;
use {CocoaObject, ObjcObjectBase, NSObject};

/// An object that manages image-based content and allows you to perform animations on that content.
#[derive(ObjcObjectBase)] pub struct CALayer(Object); DeclareClassDerivative!(CALayer : NSObject);
impl CALayer
{
    pub fn set_contents_scale(&self, scale: ::CGFloat) {
        let _: () = unsafe { msg_send![self.objid(), setContentsScale: scale] };
    }
    pub fn set_needs_display_on_bounds_change(&self, v: bool) {
        unsafe { msg_send![self.objid(), setNeedsDisplayOnBoundsChange: if v { YES } else { NO }] }
    }
    pub fn set_opaque(&self, c: bool) { unsafe { msg_send![self.objid(), setOpaque: if c { YES } else { NO }] } }
}
/// A layer that manages a pool of Metal drawables.
#[derive(ObjcObjectBase)] pub struct CAMetalLayer(Object); DeclareClassDerivative!(CAMetalLayer : CALayer);
impl CAMetalLayer {
    pub fn layer() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![Class::get("CAMetalLayer").unwrap(), layer]) }
    }
}

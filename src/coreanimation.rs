//! Core Animation

use super::{CGFloat, CGRect};
use crate::{CocoaObject, NSObject};
use objc::runtime::*;
use objc_ext::ObjcObject;

// An object that manages image-based content and allows you to perform animations on that content.
objc_ext::DefineObjcObjectWrapper!(pub CALayer : NSObject);
impl CALayer {
    pub fn set_contents_scale(&self, scale: CGFloat) {
        unsafe { msg_send![self.as_id(), setContentsScale: scale] }
    }

    pub fn set_needs_display_on_bounds_change(&mut self, v: bool) {
        unsafe { msg_send![self.as_id_mut(), setNeedsDisplayOnBoundsChange: if v { YES } else { NO }] }
    }

    pub fn set_opaque(&mut self, c: bool) {
        unsafe { msg_send![self.as_id_mut(), setOpaque: if c { YES } else { NO }] }
    }

    /// Sets the layer's frame rectangle.
    pub fn set_frame(&mut self, rect: CGRect) {
        unsafe { msg_send![self.as_id_mut(), setFrame: rect] }
    }

    /// Sets the layer's bounds rectangle.
    pub fn set_bounds(&mut self, rect: CGRect) {
        unsafe { msg_send![self.as_id_mut(), setBounds: rect] }
    }
}

objc_ext::DefineObjcObjectWrapper!(pub CAMetalLayer : CALayer);
impl CAMetalLayer {
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(CAMetalLayer), layer]) }
    }
}

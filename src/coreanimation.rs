//! Core Animation

use super::{CGFloat, CGRect};
use crate::{CocoaMutableObject, NSObject};
use objc::{class, msg_send, runtime::*, sel, sel_impl};

objc_ext::DefineObjcObjectWrapper! {
    /// An object that manages image-based content and allows you to perform animations on that content.
    pub CALayer : NSObject;
}
impl CALayer {
    #[inline(always)]
    pub fn set_contents_scale(&mut self, scale: CGFloat) {
        unsafe { msg_send![self, setContentsScale: scale] }
    }

    #[inline(always)]
    pub fn set_needs_display_on_bounds_change(&mut self, v: bool) {
        unsafe { msg_send![self, setNeedsDisplayOnBoundsChange: if v { YES } else { NO }] }
    }

    #[inline(always)]
    pub fn set_opaque(&mut self, c: bool) {
        unsafe { msg_send![self, setOpaque: if c { YES } else { NO }] }
    }

    /// Sets the layer's frame rectangle.
    #[inline(always)]
    pub fn set_frame(&mut self, rect: CGRect) {
        unsafe { msg_send![self, setFrame: rect] }
    }

    /// Sets the layer's bounds rectangle.
    #[inline(always)]
    pub fn set_bounds(&mut self, rect: CGRect) {
        unsafe { msg_send![self, setBounds: rect] }
    }
}

objc_ext::DefineObjcObjectWrapper!(pub CAMetalLayer : CALayer);
unsafe impl Sync for CAMetalLayer {}
unsafe impl Send for CAMetalLayer {}
impl CAMetalLayer {
    #[inline(always)]
    pub fn new() -> Result<CocoaMutableObject<Self>, ()> {
        unsafe { CocoaMutableObject::from_retained_id(msg_send![class!(CAMetalLayer), layer]).ok_or(()) }
    }
}

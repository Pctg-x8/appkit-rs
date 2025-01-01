//! AppKit bindings

use crate::{
    CALayer, CGColor, CGColorRef, CGFloat, CGPoint, CGRect, CGSize, CocoaMutableObject, CocoaObject, NSInteger,
    NSObject, NSString, NSUInteger,
};
use bitflags::bitflags;
use objc::{class, msg_send, runtime::*, sel, sel_impl};
use objc_ext::ObjcObject;
use std::mem::zeroed;

type NSRunLoopMode = *mut Object;

#[link(name = "AppKit", kind = "framework")]
unsafe extern "system" {
    #[allow(improper_ctypes)]
    pub unsafe static NSFontAttributeName: *mut NSString;
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "system" {
    pub unsafe static NSDefaultRunLoopMode: NSRunLoopMode;
}

pub type NSSize = CGSize;
pub type NSRect = CGRect;

#[repr(C)]
#[allow(dead_code)]
pub enum NSApplicationActivationPolicy {
    Regular,
    Accessory,
    Prohibited,
}
bitflags! {
    pub struct NSWindowStyleMask: NSUInteger {
        const BORDERLESS = 0;
        const TITLED = 1 << 0;
        const CLOSABLE = 1 << 1;
        const MINIATURIZABLE = 1 << 2;
        const RESIZABLE = 1 << 3;

        const TEXTURED_BACKGROUND = 1 << 8;
        const UNIFIED_TITLE_AND_TOOLBAR = 1 << 12;
        // >= OS X 10.7
        const FULLSCREEN = 1 << 14;
        // >= OS X 10.10
        const FULLSIZE_CONTENT_VIEW = 1 << 15;

        const UTILITY_WINDOW = 1 << 4;
        const DOC_MODAL_WINDOW = 1 << 6;
        const NONACTIVATING_PANEL = 1 << 7;
        // >= OS X 10.6
        const HUD_WINDOW = 1 << 13;
    }
}
bitflags! {
    pub struct NSEventModifierFlags : NSUInteger {
        const COMMAND = 1 << 20;
        const OPTION = 1 << 19;
        const SHIFT = 1 << 17;
    }
}

objc_ext::DefineObjcObjectWrapper!(pub NSApplication : NSObject);
impl NSApplication {
    #[inline(always)]
    pub fn shared() -> Option<&'static Self> {
        let p: *mut Object = unsafe { msg_send![class!(NSApplication), sharedApplication] };
        unsafe { (p as *const Self).as_ref() }
    }

    #[inline(always)]
    pub fn shared_mut() -> Option<&'static mut Self> {
        let p: *mut Object = unsafe { msg_send![class!(NSApplication), sharedApplication] };
        unsafe { (p as *mut Self).as_mut() }
    }

    #[inline(always)]
    pub fn set_activation_policy(&self, policy: NSApplicationActivationPolicy) -> bool {
        let b: BOOL = unsafe { msg_send![self, setActivationPolicy: policy as NSInteger] };
        b == YES
    }

    #[inline(always)]
    pub fn run(&self) {
        unsafe { msg_send![self, run] }
    }

    #[inline(always)]
    pub fn stop(&self, sender: &Object) {
        unsafe { msg_send![self, stop: sender] }
    }

    #[inline(always)]
    pub fn activate_ignoring_other_apps(&self) {
        unsafe { msg_send![self, activateIgnoringOtherApps: YES] }
    }

    #[inline(always)]
    pub fn set_delegate(&self, delegate: &Object) {
        unsafe { msg_send![self, setDelegate: delegate] }
    }

    #[inline(always)]
    pub fn set_main_menu(&mut self, menu: &NSMenu) {
        unsafe { msg_send![self, setMainMenu: menu] }
    }

    #[inline(always)]
    pub fn reply_to_application_should_terminate(&self, should_terminate: bool) {
        unsafe { msg_send![self, replyToApplicationShouldTerminate: if should_terminate { YES } else { NO }] }
    }

    #[inline(always)]
    pub fn post_event(&self, event: &NSEvent, at_start: bool) {
        unsafe { msg_send![self, postEvent: event atStart: if at_start { YES } else { NO }] }
    }
}

objc_ext::DefineObjcObjectWrapper!(pub NSWindow : NSResponder);
impl NSWindow {
    #[inline(always)]
    fn alloc() -> *mut Object {
        unsafe { msg_send![class!(NSWindow), alloc] }
    }

    #[inline(always)]
    pub fn new(content_rect: NSRect, style_mask: NSWindowStyleMask) -> Result<CocoaMutableObject<Self>, ()> {
        unsafe {
            CocoaMutableObject::from_retained_id(msg_send![Self::alloc(), initWithContentRect: content_rect styleMask: style_mask.bits() backing: 2 defer: YES])
            .ok_or(())
        }
    }

    #[inline(always)]
    pub fn with_view_controller(vc: &mut NSViewController) -> Result<CocoaMutableObject<Self>, ()> {
        unsafe {
            CocoaMutableObject::from_retained_id(msg_send![class!(NSWindow), windowWithContentViewController: vc])
                .ok_or(())
        }
    }

    #[inline(always)]
    pub fn center(&self) {
        unsafe { msg_send![self, center] }
    }

    #[inline(always)]
    pub fn make_key_and_order_front(&self, sender: &(impl ObjcObject + ?Sized)) {
        unsafe { msg_send![self, makeKeyAndOrderFront: sender.as_id()] }
    }

    #[inline(always)]
    pub fn make_main_window(&self) {
        unsafe { msg_send![self, makeMainWindow] }
    }

    #[inline(always)]
    pub fn set_title(&self, title: &NSString) {
        unsafe { msg_send![self, setTitle: title] }
    }

    #[inline(always)]
    pub fn set_alpha_value(&self, a: CGFloat) {
        unsafe { msg_send![self, setAlphaValue: a] }
    }

    #[inline(always)]
    pub fn set_background_color(&self, bg: &NSColor) {
        let _: () = unsafe { msg_send![self, setBackgroundColor: bg] };
    }

    #[inline(always)]
    pub fn set_opaque(&self, op: bool) {
        unsafe { msg_send![self, setOpaque: if op { YES } else { NO }] }
    }

    #[inline(always)]
    pub fn content_view(&self) -> &NSView {
        unsafe { msg_send![self, contentView] }
    }

    #[inline(always)]
    pub fn content_view_mut(&mut self) -> &mut NSView {
        unsafe { msg_send![self, contentView] }
    }

    #[inline(always)]
    pub fn set_content_view(&mut self, content_view: &NSView) {
        unsafe { msg_send![self, setContentView: content_view] }
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// An object that manages an app's menus.
    pub NSMenu : NSObject;
}
impl NSMenu {
    #[inline(always)]
    pub fn new() -> Result<CocoaMutableObject<Self>, ()> {
        unsafe { CocoaMutableObject::from_retained_id(msg_send![class!(NSMenu), new]).ok_or(()) }
    }

    /// Adds a menu item to the end of the menu.
    #[inline(always)]
    pub fn add(&mut self, item: &NSMenuItem) -> &mut Self {
        let _: () = unsafe { msg_send![self, addItem: item] };
        self
    }

    /// Creates a new menu item and adds it to the end of the menu.
    #[inline]
    pub fn add_new_item(
        &mut self,
        title: &NSString,
        action: Option<Sel>,
        key_equivalent: Option<&NSString>,
    ) -> Result<&mut NSMenuItem, ()> {
        let action = action.unwrap_or(unsafe { zeroed() });
        let k = key_equivalent.unwrap_or_else(|| NSString::empty());

        let item: *mut Object = unsafe {
            msg_send![self, addItemWithTitle: title action: action keyEquivalent: k as *const _ as *const Object]
        };
        let item = item as *mut NSMenuItem;
        unsafe { item.as_mut().ok_or(()) }
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// A command item in an app menu.
    pub NSMenuItem : NSObject;
}
impl NSMenuItem {
    #[inline(always)]
    fn alloc() -> *mut Object {
        unsafe { msg_send![class!(NSMenuItem), alloc] }
    }

    /// Returns an initialized instance of `NSMenuItem`.
    #[inline]
    pub fn new(
        title: &NSString,
        action: Option<Sel>,
        key_equivalent: Option<&NSString>,
    ) -> Result<CocoaMutableObject<Self>, ()> {
        let action = action.unwrap_or(unsafe { zeroed() });
        let k = key_equivalent.unwrap_or_else(|| NSString::empty());

        unsafe {
            CocoaMutableObject::from_retained_id(
                msg_send![Self::alloc(), initWithTitle: title action: action keyEquivalent: k],
            )
            .ok_or(())
        }
    }

    /// Returns a menu item that is used to separate logical groups of menu commands.
    #[inline]
    pub fn separator() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSMenuItem), separatorItem] };

        if p.is_null() {
            Err(())
        } else {
            Ok(unsafe { &*(p as *const _) })
        }
    }

    /// Sets the submenu of the menu item.
    #[inline(always)]
    pub fn set_submenu(&mut self, sub: &NSMenu) -> &mut Self {
        let _: () = unsafe { msg_send![self, setSubmenu: sub] };

        self
    }

    /// Sets the menu item's target.
    #[inline(always)]
    pub fn set_target(&mut self, target: &(impl ObjcObject + ?Sized)) -> &mut Self {
        let _: () = unsafe { msg_send![self, setTarget: target.as_id()] };

        self
    }

    /// Sets the menu item's unmodified key equivalent.
    #[inline(always)]
    pub fn set_key_equivalent_modifier_mask(&mut self, mods: NSEventModifierFlags) -> &mut Self {
        let _: () = unsafe { msg_send![self, setKeyEquivalentModifierMask: mods.bits] };

        self
    }

    /// Sets the menu item's keyboard equivalent modifiers.
    #[inline(always)]
    pub fn set_key_equivalent(&mut self, k: &NSString) -> &mut Self {
        let _: () = unsafe { msg_send![self, setKeyEquivalent: k] };

        self
    }

    /// Sets the menu item's key equivalents with modifiers.
    #[inline(always)]
    pub fn set_accelerator(&mut self, mods: NSEventModifierFlags, key: &NSString) -> &mut Self {
        self.set_key_equivalent(key).set_key_equivalent_modifier_mask(mods)
    }

    /// Sets the menu item's action-method selector.
    #[inline(always)]
    pub fn set_action(&mut self, sel: Sel) -> &mut Self {
        let _: () = unsafe { msg_send![self, setAction: sel] };

        self
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// The infrastructure for drawing, printing, and handling events in an app.
    pub NSView : NSResponder;
}
impl NSView {
    /// The Core Animation layer that the view uses as its backing store.
    #[inline(always)]
    pub fn layer(&self) -> Option<&CALayer> {
        let p: *mut Object = unsafe { msg_send![self, layer] };
        unsafe { (p as *const CALayer).as_ref() }
    }

    /// The Core Animation layer that the view uses as its backing store.
    #[inline(always)]
    pub fn layer_mut(&mut self) -> Option<&mut CALayer> {
        let p: *mut Object = unsafe { msg_send![self, layer] };
        unsafe { (p as *mut CALayer).as_mut() }
    }

    /// Sets the Core Animation layer that the view uses as its backing store.
    #[inline(always)]
    pub fn set_layer(&mut self, layer: &CALayer) {
        unsafe { msg_send![self, setLayer: layer] }
    }

    /// Sets a boolean value indicating whether the view uses a layer as its backing store.
    #[inline(always)]
    pub fn set_wants_layer(&mut self, flag: bool) {
        unsafe { msg_send![self, setWantsLayer: flag as BOOL] }
    }

    /// Sets the contents redraw policy for the view's layer.
    #[inline(always)]
    pub fn set_layer_contents_redraw_policy(&mut self, value: isize) {
        unsafe { msg_send![self, setLayerContentsRedrawPolicy: value] }
    }

    /// Sets a boolean value that determines whether the view needs to be redrawn before being displayed.
    #[inline(always)]
    pub fn set_needs_display(&mut self, flag: bool) {
        unsafe { msg_send![self, setNeedsDisplay: flag as BOOL] }
    }

    /// Sets the view's frame rectangle, which defines its position and size in its superview's coordinate system.
    #[inline(always)]
    pub fn set_frame(&mut self, f: NSRect) {
        unsafe { msg_send![self, setFrame: f] }
    }

    /// Gets the view's frame rectangle, which defines its position and size in its superview's coordinate system.
    #[inline(always)]
    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self, frame] }
    }

    /// Converts a size from the view's interior coordinate system to its pixel aligned backing store coordinate system.
    #[inline(always)]
    pub fn convert_size_to_backing(&self, size: NSSize) -> NSSize {
        unsafe { msg_send![self, convertSizeToBacking: size] }
    }

    /// Sets a boolean value indicating whether the view fills its frame rectangle with opaque content.
    #[inline(always)]
    pub fn set_opaque(&mut self, c: bool) {
        unsafe { msg_send![self, setOpaque: if c { YES } else { NO }] }
    }

    /// A boolean value indicating whether the view is being rendered as part of a live resizing operation.
    #[inline(always)]
    pub fn in_live_resize(&self) -> bool {
        let b: BOOL = unsafe { msg_send![self, inLiveResize] };
        b == YES
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// A controller that manages a view, typically loaded from a nib file.
    pub NSViewController : NSResponder;
}
impl NSViewController {
    /// The view controller's primary view.
    #[inline(always)]
    pub fn view(&self) -> Option<&NSView> {
        unsafe {
            let p: *mut Object = msg_send![self, view];
            (p as *const NSView).as_ref()
        }
    }

    /// The view controller's primary view.
    #[inline(always)]
    pub fn view_mut(&mut self) -> Option<&mut NSView> {
        unsafe {
            let p: *mut Object = msg_send![self, view];
            (p as *mut NSView).as_mut()
        }
    }

    /// Sets the view controller's primary view.
    #[inline(always)]
    pub fn set_view(&mut self, view: &NSView) {
        let _: () = unsafe { msg_send![self, setView: view] };
    }

    /// The localized title of the receiver's primary view.
    #[inline(always)]
    pub fn title(&self) -> Option<&NSString> {
        unsafe {
            let p: *mut Object = msg_send![self, title];
            (p as *const NSString).as_ref()
        }
    }

    /// Sets the localized title of the receiver's primary view.
    #[inline(always)]
    pub fn set_title(&self, title: &NSString) {
        unsafe { msg_send![self, setTitle: title] }
    }
}

/*pub struct NSRunLoop(*mut Object);
impl NSRunLoop
{
    pub fn main() -> Option<NSRunLoop>
    {
        let p: *mut Object = unsafe { msg_send![Class::get("NSRunLoop").expect("NSRunLoop"), mainRunLoop] };
        let p: *mut Object = unsafe { msg_send![p, retain] };
        if p.is_null() { None } else { Some(NSRunLoop(p)) }
    }
}
impl Drop for NSRunLoop { fn drop(&mut self) { unsafe { msg_send![self.0, release] } } }*/

objc_ext::DefineObjcObjectWrapper!(pub NSColor : NSObject);
impl NSColor {
    #[inline(always)]
    pub fn clear_color() -> Option<&'static Self> {
        let p: *mut Object = unsafe { msg_send![class!(NSColor), clearColor] };
        unsafe { (p as *const Self).as_ref() }
    }

    /// The Core Graphics color object corresponding to the color.
    #[inline(always)]
    pub fn cgcolor(&self) -> &CGColor {
        unsafe {
            let p: *mut Object = msg_send![self, CGColor];
            &*(p as CGColorRef)
        }
    }
}

objc_ext::DefineObjcObjectWrapper! {
    /// The representation of a font in an app.
    pub NSFont : NSObject;
}
impl NSFont {
    /// Creates a font object for the specified font name and font size.
    #[inline(always)]
    pub fn with_name(name: &NSString, size: CGFloat) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_retained_id(msg_send![class!(NSFont), fontWithName: name size: size]).ok_or(()) }
    }

    /// Returns the font used by default for documents and other text under the user's control
    /// (that is, text whose font the user can normally change), in the specified size.
    #[inline(always)]
    pub fn user<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), userFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the Aqua system font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size.
    #[inline(always)]
    pub fn system<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), systemFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the Aqua system font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size and the specified weight.
    #[inline(always)]
    pub fn system_with_weight<'a>(size: CGFloat, weight: NSFontWeight) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), systemFontOfSize: size weight: weight] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size.
    #[inline(always)]
    pub fn message<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), messageFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the font used for standard interface labels in the specified size.
    #[inline(always)]
    pub fn label<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), labelFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// The point size of the font.
    #[inline(always)]
    pub fn point_size(&self) -> CGFloat {
        unsafe { msg_send![self, pointSize] }
    }

    /// Returns the size of the standard system font.
    #[inline(always)]
    pub fn system_font_size() -> CGFloat {
        unsafe { msg_send![class!(NSFont), systemFontSize] }
    }

    /// Returns the size of the standard label font.
    #[inline(always)]
    pub fn label_font_size() -> CGFloat {
        unsafe { msg_send![class!(NSFont), labelFontSize] }
    }
}

/// System-defined font-weight values.
pub type NSFontWeight = CGFloat;

objc_ext::DefineObjcObjectWrapper! {
    /// An object that describes the attributes of a computer's monitor or screen.
    pub NSScreen : NSObject;
}
impl NSScreen {
    /// Returns the screen object containing the window with the keyboard focus.
    #[inline(always)]
    pub fn main() -> &'static Self {
        let p: *mut Object = unsafe { msg_send![class!(NSScreen), mainScreen] };
        unsafe { (p as *const Self).as_ref().unwrap() }
    }

    /// The backing store pixel scale factor for the screen.
    #[inline(always)]
    pub fn backing_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![self, backingScaleFactor] }
    }
}

objc_ext::DefineObjcObjectWrapper!(pub NSResponder : NSObject);

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NSEventType {
    ApplicationDefined = 15,
}
pub type NSTimeInterval = core::ffi::c_double;
objc_ext::DefineObjcObjectWrapper!(pub NSEvent : NSObject);
impl NSEvent {
    pub fn new_other_event(
        ty: NSEventType,
        location: CGPoint,
        modifier_flags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        window_number: NSInteger,
        context: Option<&Object>,
        subtype: core::ffi::c_short,
        data1: NSInteger,
        data2: NSInteger,
    ) -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_retained_id(
                msg_send![class!(NSEvent), otherEventWithType: ty as NSUInteger location: location modifierFlags: modifier_flags.bits() timestamp: timestamp windowNumber: window_number context: context subtype: subtype data1: data1 data2: data2],
            ).ok_or(())
        }
    }
}

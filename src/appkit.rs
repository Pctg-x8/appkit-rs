//! AppKit bindings

use crate::{
    CALayer, CGColor, CGColorRef, CGFloat, CGPoint, CGRect, CGSize, CocoaObject, CocoaString, NSInteger, NSObject,
    NSString, NSUInteger,
};
use objc::runtime::*;
use objc_ext::ObjcObject;
use std::mem::zeroed;

type NSRunLoopMode = *mut Object;

#[link(name = "AppKit", kind = "framework")]
extern "system" {
    #[allow(improper_ctypes)]
    pub static NSFontAttributeName: *mut NSString;
}

#[link(name = "Foundation", kind = "framework")]
extern "system" {
    pub static NSDefaultRunLoopMode: NSRunLoopMode;
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
    pub fn shared() -> Option<&'static Self> {
        let p: *mut Object = unsafe { msg_send![class!(NSApplication), sharedApplication] };
        unsafe { (p as *const Self).as_ref() }
    }

    pub fn shared_mut() -> Option<&'static mut Self> {
        let p: *mut Object = unsafe { msg_send![class!(NSApplication), sharedApplication] };
        unsafe { (p as *mut Self).as_mut() }
    }

    pub fn set_activation_policy(&self, policy: NSApplicationActivationPolicy) -> bool {
        let b: BOOL = unsafe { msg_send![self.as_id(), setActivationPolicy: policy as NSInteger] };
        b == YES
    }

    pub fn run(&self) {
        unsafe { msg_send![self.as_id(), run] }
    }

    pub fn stop(&self, sender: &objc::runtime::Object) {
        unsafe { msg_send![self.as_id(), stop:sender] }
    }

    pub fn activate_ignoring_other_apps(&self) {
        unsafe { msg_send![self.as_id(), activateIgnoringOtherApps: YES] }
    }

    pub fn set_delegate(&self, delegate: &Object) {
        unsafe { msg_send![self.as_id(), setDelegate: delegate] }
    }

    pub fn set_main_menu(&mut self, menu: &NSMenu) {
        unsafe { msg_send![self.as_id_mut(), setMainMenu: menu.as_id()] }
    }

    pub fn reply_to_application_should_terminate(&self, should_terminate: bool) {
        unsafe {
            msg_send![self.as_id(), replyToApplicationShouldTerminate:if should_terminate { objc::runtime::YES } else { objc::runtime::NO }]
        }
    }

    pub fn post_event(&self, event: &NSEvent, at_start: bool) {
        unsafe {
            msg_send![self.as_id(), postEvent: event.as_id() atStart: if at_start { objc::runtime::YES } else { objc::runtime::NO }]
        }
    }
}

objc_ext::DefineObjcObjectWrapper!(pub NSWindow : NSResponder);
impl NSWindow {
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSWindow), alloc] };
        if p.is_null() {
            Err(())
        } else {
            Ok(p)
        }
    }

    pub fn new(content_rect: NSRect, style_mask: NSWindowStyleMask) -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_id(msg_send![Self::alloc()?,
                initWithContentRect: content_rect styleMask: style_mask.bits() backing: 2 defer: YES])
        }
    }

    pub fn with_view_controller(vc: &mut NSViewController) -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSWindow), windowWithContentViewController: vc.as_id_mut()]) }
    }

    pub fn center(&self) {
        unsafe { msg_send![self.as_id(), center] }
    }

    pub fn make_key_and_order_front(&self, sender: &(impl ObjcObject + ?Sized)) {
        unsafe { msg_send![self.as_id(), makeKeyAndOrderFront: sender.as_id()] }
    }

    pub fn make_main_window(&self) {
        unsafe { msg_send![self.as_id(), makeMainWindow] }
    }

    pub fn set_title(&self, title: &(impl CocoaString + ?Sized)) {
        unsafe { msg_send![self.as_id(), setTitle: title.to_nsstring().id()] }
    }

    pub fn set_alpha_value(&self, a: CGFloat) {
        unsafe { msg_send![self.as_id(), setAlphaValue: a] }
    }

    pub fn set_background_color(&self, bg: &NSColor) {
        let _: () = unsafe { msg_send![self.as_id(), setBackgroundColor: bg.as_id()] };
    }

    pub fn set_opaque(&self, op: bool) {
        unsafe { msg_send![self.as_id(), setOpaque: if op { YES } else { NO }] }
    }

    pub fn content_view(&self) -> &NSView {
        unsafe { msg_send![self.as_id(), contentView] }
    }

    pub fn content_view_mut(&mut self) -> &mut NSView {
        unsafe { msg_send![self.as_id_mut(), contentView] }
    }

    pub fn set_content_view(&mut self, content_view: &NSView) {
        unsafe { msg_send![self.as_id_mut(), setContentView: content_view.as_id()] }
    }
}

// An object that manages an app's menus.
objc_ext::DefineObjcObjectWrapper!(pub NSMenu : NSObject);
impl NSMenu {
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![class!(NSMenu), new]) }
    }
    /// Adds a menu item to the end of the menu.
    pub fn add(&mut self, item: &NSMenuItem) -> &mut Self {
        let _: () = unsafe { msg_send![self.as_id(), addItem: item.as_id()] };
        self
    }
    /// Creates a new menu item and adds it to the end of the menu.
    pub fn add_new_item(
        &mut self,
        title: &(impl CocoaString + ?Sized),
        action: Option<Sel>,
        key_equivalent: Option<&NSString>,
    ) -> Result<&mut NSMenuItem, ()> {
        let (title, action) = (title.to_nsstring(), action.unwrap_or(unsafe { zeroed() }));
        let k = key_equivalent.unwrap_or_else(|| NSString::empty());

        let item: *mut Object = unsafe {
            msg_send![self.as_id(), addItemWithTitle: title.id() action: action keyEquivalent: k as *const _ as *const Object]
        };
        let item = item as *mut NSMenuItem;
        unsafe { item.as_mut().ok_or(()) }
    }
}
// A command item in an app menu.
objc_ext::DefineObjcObjectWrapper!(pub NSMenuItem : NSObject);
impl NSMenuItem {
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSMenuItem), alloc] };

        if p.is_null() {
            Err(())
        } else {
            Ok(p)
        }
    }

    /// Returns an initialized instance of `NSMenuItem`.
    pub fn new(
        title: &(impl CocoaString + ?Sized),
        action: Option<Sel>,
        key_equivalent: Option<&NSString>,
    ) -> Result<CocoaObject<Self>, ()> {
        let (title, action) = (title.to_nsstring(), action.unwrap_or(unsafe { zeroed() }));
        let k = key_equivalent.unwrap_or_else(|| NSString::empty());

        unsafe {
            CocoaObject::from_id(msg_send![Self::alloc()?,
                initWithTitle: title.id() action: action keyEquivalent: k.as_id()])
        }
    }

    /// Returns a menu item that is used to separate logical groups of menu commands.
    pub fn separator() -> Result<&'static Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSMenuItem), separatorItem] };
        if p.is_null() {
            return Err(());
        }

        Ok(unsafe { &*(p as *const _) })
    }

    /// Sets the submenu of the menu item.
    pub fn set_submenu(&mut self, sub: &NSMenu) -> &mut Self {
        let _: () = unsafe { msg_send![self.as_id_mut(), setSubmenu: sub.as_id()] };

        self
    }

    /// Sets the menu item's target.
    pub fn set_target(&mut self, target: &(impl ObjcObject + ?Sized)) -> &mut Self {
        let _: () = unsafe { msg_send![self.as_id_mut(), setTarget: target.as_id()] };

        self
    }

    /// Sets the menu item's unmodified key equivalent.
    pub fn set_key_equivalent_modifier_mask(&mut self, mods: NSEventModifierFlags) -> &mut Self {
        let _: () = unsafe { msg_send![self.as_id_mut(), setKeyEquivalentModifierMask: mods.bits] };

        self
    }

    /// Sets the menu item's keyboard equivalent modifiers.
    pub fn set_key_equivalent(&mut self, k: &(impl CocoaString + ?Sized)) -> &mut Self {
        let _: () = unsafe { msg_send![self.as_id_mut(), setKeyEquivalent: k.to_nsstring().id()] };

        self
    }

    /// Sets the menu item's key equivalents with modifiers.
    pub fn set_accelerator(&mut self, mods: NSEventModifierFlags, key: &(impl CocoaString + ?Sized)) -> &mut Self {
        self.set_key_equivalent(key).set_key_equivalent_modifier_mask(mods)
    }

    /// Sets the menu item's action-method selector.
    pub fn set_action(&mut self, sel: Sel) -> &mut Self {
        let _: () = unsafe { msg_send![self.as_id_mut(), setAction: sel] };

        self
    }
}

// The infrastructure for drawing, printing, and handling events in an app.
objc_ext::DefineObjcObjectWrapper!(pub NSView : NSResponder);
impl NSView {
    /// The Core Animation layer that the view uses as its backing store.
    pub fn layer(&self) -> Option<&CALayer> {
        let p: *mut Object = unsafe { msg_send![self.as_id(), layer] };
        unsafe { (p as *const CALayer).as_ref() }
    }

    /// The Core Animation layer that the view uses as its backing store.
    pub fn layer_mut(&mut self) -> Option<&mut CALayer> {
        let p: *mut Object = unsafe { msg_send![self.as_id_mut(), layer] };
        unsafe { (p as *mut CALayer).as_mut() }
    }

    /// Sets the Core Animation layer that the view uses as its backing store.
    pub fn set_layer(&mut self, layer: &(impl ObjcObject + ?Sized)) {
        unsafe { msg_send![self.as_id_mut(), setLayer: layer.as_id()] }
    }

    /// Sets a boolean value indicating whether the view uses a layer as its backing store.
    pub fn set_wants_layer(&mut self, flag: bool) {
        unsafe { msg_send![self.as_id_mut(), setWantsLayer: flag as BOOL] }
    }

    /// Sets the contents redraw policy for the view's layer.
    pub fn set_layer_contents_redraw_policy(&mut self, value: isize) {
        unsafe { msg_send![self.as_id_mut(), setLayerContentsRedrawPolicy: value] }
    }

    /// Sets a boolean value that determines whether the view needs to be redrawn before being displayed.
    pub fn set_needs_display(&mut self, flag: bool) {
        unsafe { msg_send![self.as_id_mut(), setNeedsDisplay: flag as BOOL] }
    }

    /// Sets the view's frame rectangle, which defines its position and size in its superview's coordinate system.
    pub fn set_frame(&mut self, f: &NSRect) {
        unsafe { msg_send![self.as_id_mut(), setFrame: f.clone()] }
    }

    /// Gets the view's frame rectangle, which defines its position and size in its superview's coordinate system.
    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self.as_id(), frame] }
    }

    /// Converts a size from the view's interior coordinate system to its pixel aligned backing store coordinate system.
    pub fn convert_size_to_backing(&self, size: &NSSize) -> NSSize {
        unsafe { msg_send![self.as_id(), convertSizeToBacking: size.clone()] }
    }

    /// Sets a boolean value indicating whether the view fills its frame rectangle with opaque content.
    pub fn set_opaque(&mut self, c: bool) {
        unsafe { msg_send![self.as_id_mut(), setOpaque: if c { YES } else { NO }] }
    }

    /// A boolean value indicating whether the view is being rendered as part of a live resizing operation.
    pub fn in_live_resize(&self) -> bool {
        let b: BOOL = unsafe { msg_send![self.as_id(), inLiveResize] };
        b == YES
    }
}

// A controller that manages a view, typically loaded from a nib file.
objc_ext::DefineObjcObjectWrapper!(pub NSViewController : NSResponder);
impl NSViewController {
    /// The view controller's primary view.
    pub fn view(&self) -> Option<&NSView> {
        unsafe {
            let p: *mut Object = msg_send![self.as_id(), view];
            (p as *const NSView).as_ref()
        }
    }

    /// The view controller's primary view.
    pub fn view_mut(&mut self) -> Option<&mut NSView> {
        unsafe {
            let p: *mut Object = msg_send![self.as_id(), view];
            (p as *mut NSView).as_mut()
        }
    }

    /// Sets the view controller's primary view.
    pub fn set_view(&mut self, view: &NSView) {
        let _: () = unsafe { msg_send![&mut self.0, setView: view.as_id()] };
    }

    /// The localized title of the receiver's primary view.
    pub fn title(&self) -> Option<&NSString> {
        unsafe {
            let p: *mut Object = msg_send![&self.0, title];
            (p as *const NSString).as_ref()
        }
    }

    /// Sets the localized title of the receiver's primary view.
    pub fn set_title(&self, title: &(impl CocoaString + ?Sized)) {
        unsafe { msg_send![&self.0, setTitle: title.to_nsstring().id()] }
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
    pub fn clear_color() -> Option<&'static Self> {
        let p: *mut Object = unsafe { msg_send![class!(NSColor), clearColor] };
        unsafe { (p as *const Self).as_ref() }
    }

    /// The Core Graphics color object corresponding to the color.
    pub fn cgcolor(&self) -> &CGColor {
        unsafe {
            let p: *mut Object = msg_send![self.as_id(), CGColor];
            &*(p as CGColorRef)
        }
    }
}

// The representation of a font in an app.
objc_ext::DefineObjcObjectWrapper!(pub NSFont : NSObject);
impl NSFont {
    /// Creates a font object for the specified font name and font size.
    pub fn with_name<'a>(name: &(impl CocoaString + ?Sized), size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), fontWithName: name.to_nsstring().as_id() size: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the font used by default for documents and other text under the user's control
    /// (that is, text whose font the user can normally change), in the specified size.
    pub fn user<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), userFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the Aqua system font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size.
    pub fn system<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), systemFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the Aqua system font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size and the specified weight.
    pub fn system_with_weight<'a>(size: CGFloat, weight: NSFontWeight) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), systemFontOfSize: size weight: weight] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size.
    pub fn message<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), messageFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// Returns the font used for standard interface labels in the specified size.
    pub fn label<'a>(size: CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![class!(NSFont), labelFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }

    /// The point size of the font.
    pub fn point_size(&self) -> CGFloat {
        unsafe { msg_send![self.as_id(), pointSize] }
    }

    /// Returns the size of the standard system font.
    pub fn system_font_size() -> CGFloat {
        unsafe { msg_send![class!(NSFont), systemFontSize] }
    }

    /// Returns the size of the standard label font.
    pub fn label_font_size() -> CGFloat {
        unsafe { msg_send![class!(NSFont), labelFontSize] }
    }
}
/// System-defined font-weight values.
pub type NSFontWeight = CGFloat;

// An object that describes the attributes of a computer's monitor or screen.
objc_ext::DefineObjcObjectWrapper!(pub NSScreen : NSObject);
impl NSScreen {
    /// Returns the screen object containing the window with the keyboard focus.
    pub fn main() -> &'static Self {
        let p: *mut Object = unsafe { msg_send![class!(NSScreen), mainScreen] };
        unsafe { (p as *const Self).as_ref().unwrap() }
    }

    /// The backing store pixel scale factor for the screen.
    pub fn backing_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![self.as_id(), backingScaleFactor] }
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
        context: Option<&objc::runtime::Object>,
        subtype: core::ffi::c_short,
        data1: NSInteger,
        data2: NSInteger,
    ) -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_id(
                msg_send![class!(NSEvent), otherEventWithType: ty as NSUInteger location: location modifierFlags: modifier_flags.bits() timestamp: timestamp windowNumber: window_number context: context subtype: subtype data1: data1 data2: data2],
            )
        }
    }
}

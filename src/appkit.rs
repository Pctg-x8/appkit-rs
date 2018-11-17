//! AppKit bindings

use objc::runtime::*;
use std::mem::zeroed;
use {CocoaObject, CocoaString, NSObject, ObjcObjectBase};
use appkit_derive::ObjcObjectBase;

/*#[cfg(feature = "with_ferrite")]
type NSRunLoopMode = *mut Object;*/
#[cfg(feature = "with_ferrite")] #[cfg(not(feature = "manual_rendering"))] pub type CVOptionFlags = u64;
#[link(name = "AppKit", kind = "framework")] extern "system" {
    pub static NSFontAttributeName: *mut ::NSString;
}
/*#[cfg(feature = "with_ferrite")]
#[link(name = "Foundation", kind = "framework")] extern "system"
{
    pub static NSDefaultRunLoopMode: NSRunLoopMode;
}*/

pub type NSSize = ::CGSize;
pub type NSRect = ::CGRect;

#[repr(C)] #[allow(dead_code)]
pub enum NSApplicationActivationPolicy { Regular, Accessory, Prohibited }
bitflags! {
    pub struct NSWindowStyleMask: ::NSUInteger {
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
    pub struct NSEventModifierFlags : ::NSUInteger {
        const COMMAND = 1 << 20;
        const OPTION = 1 << 19;
        const SHIFT = 1 << 17;
    }
}

#[derive(ObjcObjectBase)]
pub struct NSApplication(Object); DeclareClassDerivative!(NSApplication : NSObject);
impl NSApplication
{
    pub fn shared() -> Option<&'static Self>
    {
        let p: *mut Object = unsafe { msg_send![Class::get("NSApplication").unwrap(), sharedApplication] };
        unsafe { (p as *const Self).as_ref() }
    }

    pub fn set_activation_policy(&self, policy: NSApplicationActivationPolicy) -> bool
    {
        let b: BOOL = unsafe { msg_send![&self.0, setActivationPolicy: policy as ::NSInteger] };
        b == YES
    }
    pub fn run(&self) { unsafe { msg_send![&self.0, run] } }
    pub fn activate_ignoring_other_apps(&self)
    {
        unsafe { msg_send![&self.0, activateIgnoringOtherApps: YES] }
    }
    pub fn set_delegate(&self, delegate: &Object)
    {
        unsafe { msg_send![&self.0, setDelegate: delegate] }
    }
    pub fn set_main_menu(&self, menu: &NSMenu)
    {
        let _: () = unsafe { msg_send![&self.0, setMainMenu: &menu.0] };
    }
}
#[derive(ObjcObjectBase)]
pub struct NSWindow(Object); DeclareClassDerivative!(NSWindow : NSObject);
impl NSWindow
{
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSWindow").unwrap(), alloc] };
        if p.is_null() { Err(()) } else { Ok(p) }
    }
    pub fn new(content_rect: NSRect, style_mask: NSWindowStyleMask) -> Result<CocoaObject<Self>, ()> {
        unsafe {
            CocoaObject::from_id(msg_send![Self::alloc()?,
                initWithContentRect: content_rect styleMask: style_mask.bits() backing: 2 defer: YES])
        }
    }
    pub unsafe fn with_view_controller_ptr(vc: *mut Object) -> Result<CocoaObject<Self>, ()> {
        CocoaObject::from_id(msg_send![Class::get("NSWindow").unwrap(), windowWithContentViewController: vc])
    }

    pub fn center(&self) { unsafe { msg_send![&self.0, center] } }
    pub fn make_key_and_order_front(&self, sender: &Object)
    {
        unsafe { msg_send![&self.0, makeKeyAndOrderFront: sender] }
    }
    pub fn set_title<Title: CocoaString + ?Sized>(&self, title: &Title)
    {
        unsafe { msg_send![&self.0, setTitle: title.to_nsstring().id()] }
    }
    pub fn set_alpha_value(&self, a: ::CGFloat) { unsafe { msg_send![&self.0, setAlphaValue: a] } }
    pub fn set_background_color(&self, bg: &NSColor)
    {
        let _: () = unsafe { msg_send![&self.0, setBackgroundColor: &bg.0] };
    }
    pub fn set_opaque(&self, op: bool)
    {
        unsafe { msg_send![&self.0, setOpaque: if op { YES } else { NO }] }
    }
}
/// An object that manages an app's menus.
#[derive(ObjcObjectBase)]
pub struct NSMenu(Object); DeclareClassDerivative!(NSMenu : NSObject);
impl NSMenu {
    pub fn new() -> Result<CocoaObject<Self>, ()> {
        unsafe { CocoaObject::from_id(msg_send![Class::get("NSMenu").unwrap(), new]) }
    }
    /// Adds a menu item to the end of the menu.
    pub fn add(&mut self, item: &NSMenuItem) -> &mut Self {
        let _: () = unsafe { msg_send![&self.0, addItem: &item.0] };
        return self;
    }
    /// Creates a new menu item and adds it to the end of the menu.
    pub fn add_new_item<T>(&mut self, title: &T, action: Option<Sel>, key_equivalent: Option<&::NSString>)
            -> Result<&mut NSMenuItem, ()> where T: CocoaString + ?Sized {
        let (title, action) = (title.to_nsstring(), action.unwrap_or(unsafe { zeroed() }));
        let k = key_equivalent.unwrap_or_else(|| ::NSString::empty());

        let item: *mut Object = unsafe {
            msg_send![&self.0, addItemWithTitle: title.id() action: action keyEquivalent: &k as *const _ as *const Object]
        };
        let item = item as *mut NSMenuItem;
        unsafe { item.as_mut().ok_or(()) }
    }
}
/// A command item in an app menu.
#[derive(ObjcObjectBase)]
pub struct NSMenuItem(Object); DeclareClassDerivative!(NSMenuItem : NSObject);
impl NSMenuItem {
    fn alloc() -> Result<*mut Object, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSMenuItem").unwrap(), alloc] };
        if p.is_null() { Err(()) } else { Ok(p) }
    }
    /// Returns an initialized instance of `NSMenuItem`.
    pub fn new<Title: CocoaString + ?Sized>(title: &Title, action: Option<Sel>, key_equivalent: Option<&::NSString>)
            -> Result<CocoaObject<Self>, ()> {
        let (title, action) = (title.to_nsstring(), action.unwrap_or(unsafe { zeroed() }));
        let k = key_equivalent.unwrap_or_else(|| ::NSString::empty());
        unsafe {
            CocoaObject::from_id(msg_send![Self::alloc()?,
                initWithTitle: title.id() action: action keyEquivalent: k.objid()])
        }
    }
    /// Returns a menu item that is used to separate logical groups of menu commands.
    pub fn separator() -> Result<CocoaObject<Self>, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSMenuItem").unwrap(), separatorItem] };
        if p.is_null() { return Err(()); }
        unsafe { CocoaObject::from_id(msg_send![p, retain]) }
    }

    /// Sets the submenu of the menu item.
    pub fn set_submenu(&self, sub: &NSMenu) -> &Self {
        let _: () = unsafe { msg_send![&self.0, setSubmenu: &sub.0] };
        return self;
    }
    /// Sets the menu item's target.
    pub fn set_target(&self, target: *mut Object) -> &Self {
        let _: () = unsafe { msg_send![&self.0, setTarget: target] };
        return self;
    }
    /// Sets the menu item's unmodified key equivalent.
    pub fn set_key_equivalent_modifier_mask(&self, mods: NSEventModifierFlags) -> &Self {
        let _: () = unsafe { msg_send![&self.0, setKeyEquivalentModifierMask: mods.bits] };
        return self;
    }
    /// Sets the menu item's keyboard equivalent modifiers.
    pub fn set_key_equivalent<Str: CocoaString + ?Sized>(&self, k: &Str) -> &Self {
        let _: () = unsafe { msg_send![&self.0, setKeyEquivalent: k.to_nsstring().id()] };
        return self;
    }
    /// Sets the menu item's key equivalents with modifiers.
    pub fn set_accelerator<Str: CocoaString + ?Sized>(&self, mods: NSEventModifierFlags, key: &Str) -> &Self {
        self.set_key_equivalent(key).set_key_equivalent_modifier_mask(mods)
    }
    /// Sets the menu item's action-method selector.
    pub fn set_action(&self, sel: Sel) -> &Self { let _: () = unsafe { msg_send![&self.0, setAction: sel] }; self }
}

/// The infrastructure for drawing, printing, and handling events in an app.
#[derive(ObjcObjectBase)] pub struct NSView(Object); DeclareClassDerivative!(NSView : NSObject);
impl NSView {
    /// The Core Animation layer that the view uses as its backing store.
    pub fn layer(&self) -> Option<&::CALayer> {
        let p: *mut Object = unsafe { msg_send![self.objid(), layer] };
        unsafe { (p as *const ::CALayer).as_ref() }
    }
    /// The Core Animation layer that the view uses as its backing store.
    pub fn layer_mut(&mut self) -> Option<&mut ::CALayer> {
        let p: *mut Object = unsafe { msg_send![self.objid_mut(), layer] };
        unsafe { (p as *mut ::CALayer).as_mut() }
    }
    /// Sets the Core Animation layer that the view uses as its backing store.
    pub fn set_layer(&mut self, layer: *mut Object) { unsafe { msg_send![self.objid_mut(), setLayer: layer] } }
    /// Sets a boolean value indicating whether the view uses a layer as its backing store.
    pub fn set_wants_layer(&mut self, flag: bool) {
        unsafe { msg_send![self.objid_mut(), setWantsLayer: flag as BOOL] }
    }
    /// Sets the contents redraw policy for the view's layer.
    pub fn set_layer_contents_redraw_policy(&mut self, value: isize) {
        unsafe { msg_send![self.objid_mut(), setLayerContentsRedrawPolicy: value] }
    }
    /// Sets a boolean value that determines whether the view needs to be redrawn before being displayed.
    pub fn set_needs_display(&mut self, flag: bool) {
        unsafe { msg_send![self.objid_mut(), setNeedsDisplay: flag as BOOL] }
    }
    /// Sets the view's frame rectangle, which defines its position and size in its superview's coordinate system.
    pub fn set_frame(&mut self, f: &NSRect) { unsafe { msg_send![self.objid_mut(), setFrame: f.clone()] } }
    /// Converts a size from the view's interior coordinate system to its pixel aligned backing store coordinate system.
    pub fn convert_size_to_backing(&self, size: &NSSize) -> NSSize {
        unsafe { msg_send![self.objid(), convertSizeToBacking:size.clone()] }
    }
    /// Sets a boolean value indicating whether the view fills its frame rectangle with opaque content.
    pub fn set_opaque(&mut self, c: bool) { 
        unsafe { msg_send![self.objid_mut(), setOpaque: if c { YES } else { NO }] }
    }
}
/// A controller that manages a view, typically loaded from a nib file.
#[derive(ObjcObjectBase)] pub struct NSViewController(Object); DeclareClassDerivative!(NSViewController : NSObject);
impl NSViewController {
    /// The view controller's primary view.
    pub fn view(&self) -> Option<&NSView> {
        unsafe { let p: *mut Object = msg_send![self.objid(), view]; (p as *const NSView).as_ref() }
    }
    /// The view controller's primary view.
    pub fn view_mut(&mut self) -> Option<&mut NSView> {
        unsafe { let p: *mut Object = msg_send![self.objid(), view]; (p as *mut NSView).as_mut() }
    }
    /// The localized title of the receiver's primary view.
    pub fn title(&self) -> Option<&::NSString> {
        unsafe { let p: *mut Object = msg_send![&self.0, title]; (p as *const ::NSString).as_ref() }
    }
    /// Sets the localized title of the receiver's primary view.
    pub fn set_title<S: CocoaString + ?Sized>(&self, title: &S) {
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

#[derive(ObjcObjectBase)] pub struct NSColor(Object); DeclareClassDerivative!(NSColor : NSObject);
impl NSColor {
    pub fn clear_color() -> Option<&'static Self> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSColor").unwrap(), clearColor] };
        unsafe { (p as *const Self).as_ref() }
    }

    /// The Core Graphics color object corresponding to the color.
    pub fn cgcolor(&self) -> &::CGColor {
        unsafe { let p: *mut Object = msg_send![self.objid(), CGColor]; &*(p as ::CGColorRef) }
    }
}

/// The representation of a font in an app.
#[derive(ObjcObjectBase)] pub struct NSFont(Object); DeclareClassDerivative!(NSFont : NSObject);
impl NSFont {
    /// Creates a font object for the specified font name and font size.
    pub fn with_name<'a, N: CocoaString + ?Sized>(name: &N, size: ::CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe {
            msg_send![Class::get("NSFont").unwrap(), fontWithName: name.to_nsstring().objid() size: size]
        };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }
    /// Returns the font used by default for documents and other text under the user's control
    /// (that is, text whose font the user can normally change), in the specified size.
    pub fn user<'a>(size: ::CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSFont").unwrap(), userFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }
    /// Returns the Aqua system font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size.
    pub fn system<'a>(size: ::CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSFont").unwrap(), systemFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }
    /// Returns the Aqua system font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size and the specified weight.
    pub fn system_with_weight<'a>(size: ::CGFloat, weight: NSFontWeight) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSFont").unwrap(), systemFontOfSize: size weight: weight] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }
    /// Returns the font used for standard interface items, such as button labels,
    /// menu items, and so on, in the specified size.
    pub fn message<'a>(size: ::CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSFont").unwrap(), messageFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }
    /// Returns the font used for standard interface labels in the specified size.
    pub fn label<'a>(size: ::CGFloat) -> Result<&'a Self, ()> {
        let p: *mut Object = unsafe { msg_send![Class::get("NSFont").unwrap(), labelFontOfSize: size] };
        unsafe { (p as *const Self).as_ref().ok_or(()) }
    }
    /// The point size of the font.
    pub fn point_size(&self) -> ::CGFloat { unsafe { msg_send![self.objid(), pointSize] } }

    /// Returns the size of the standard system font.
    pub fn system_font_size() -> ::CGFloat { unsafe { msg_send![Class::get("NSFont").unwrap(), systemFontSize] } }
    /// Returns the size of the standard label font.
    pub fn label_font_size() -> ::CGFloat { unsafe { msg_send![Class::get("NSFont").unwrap(), labelFontSize] } }
}
/// System-defined font-weight values.
pub type NSFontWeight = ::CGFloat;

/// An object that describes the attributes of a computer's monitor or screen.
#[derive(ObjcObjectBase)] pub struct NSScreen(Object); DeclareClassDerivative!(NSScreen : NSObject);
impl NSScreen {
    /// Returns the screen object containing the window with the keyboard focus.
    pub fn main() -> &'static Self {
        let p: *mut Object = unsafe { msg_send![Class::get("NSScreen").unwrap(), mainScreen] };
        unsafe { (p as * const Self).as_ref().unwrap() }
    }
    /// The backing store pixel scale factor for the screen.
    pub fn backing_scale_factor(&self) -> ::CGFloat { unsafe { msg_send![self.objid(), backingScaleFactor] } }
}

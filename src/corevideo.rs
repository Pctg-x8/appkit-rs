//! Core Video
#![allow(non_upper_case_globals)]

use crate::{CGDirectDisplayID, CoreObject, CoreRetainedMutableObject};

/// A Core Video error type return value.
pub type CVReturn = i32;
pub const kCVReturnSuccess: CVReturn = 0;

/// Defines a pointer to a display link output callback function, which is called whenever the display link wants
/// the application to output a frame.
pub type CVDisplayLinkOutputCallback = Option<
    extern "system" fn(
        displayLink: CVDisplayLinkRef,
        inNow: *const CVTimeStamp,
        inOutputTime: *const CVTimeStamp,
        flagsIn: CVOptionFlags,
        flagsOut: *mut CVOptionFlags,
        displayLinkContext: *mut core::ffi::c_void,
    ) -> CVReturn,
>;

/// A structure for defining a display timestamp.
#[repr(C)]
#[allow(non_snake_case)]
pub struct CVTimeStamp {
    pub version: u32,
    pub videoTimeScale: i32,
    pub videoTime: i64,
    pub hostTime: u64,
    pub rateScalar: core::ffi::c_double,
    pub videoRefreshPeriod: i64,
    pub smpteTime: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}

/// A structure for holding an SMPTE time.
#[repr(C)]
#[allow(non_snake_case)]
pub struct CVSMPTETime {
    pub subframes: i16,
    pub subframeDivisor: i16,
    pub counter: u32,
    pub type_: u32,
    pub flags: u32,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

/// The flags to be used for the display link output callback function.
pub type CVOptionFlags = u64;

DefineOpaqueFFIObject! {
    /// A high-priority thread that notifies your app when a given display will need each frame.
    pub struct CVDisplayLink;
}
unsafe impl CoreObject for CVDisplayLink {
    #[inline(always)]
    unsafe fn retain(ptr: *const Self) {
        CVDisplayLinkRetain(ptr as _);
    }

    #[inline(always)]
    unsafe fn release(ptr: *const Self) {
        CVDisplayLinkRelease(ptr as _);
    }
}
/// A reference to a display link object.
pub type CVDisplayLinkRef = *mut CVDisplayLink;

impl CVDisplayLink {
    /// Creates a display link capable of being used with all active displays.
    #[inline]
    pub fn new_for_active_displays() -> Result<CoreRetainedMutableObject<Self>, CVReturn> {
        let mut h = core::mem::MaybeUninit::uninit();
        let r = unsafe { CVDisplayLinkCreateWithActiveCGDisplays(h.as_mut_ptr()) };

        if r == kCVReturnSuccess {
            Ok(unsafe { CoreRetainedMutableObject::retained(core::ptr::NonNull::new_unchecked(h.assume_init())) })
        } else {
            Err(r)
        }
    }

    /// Creates a display link for a single display.
    #[inline]
    pub fn new_for_display(id: CGDirectDisplayID) -> Result<CoreRetainedMutableObject<Self>, CVReturn> {
        let mut h = core::mem::MaybeUninit::uninit();
        let r = unsafe { CVDisplayLinkCreateWithCGDisplay(id, h.as_mut_ptr()) };

        if r == kCVReturnSuccess {
            Ok(unsafe { CoreRetainedMutableObject::retained(core::ptr::NonNull::new_unchecked(h.assume_init())) })
        } else {
            Err(r)
        }
    }

    /// Sets the renderer output callback function
    #[inline]
    pub fn set_output_callback(
        &mut self,
        callback: CVDisplayLinkOutputCallback,
        user: *mut core::ffi::c_void,
    ) -> Result<(), CVReturn> {
        let r = unsafe { CVDisplayLinkSetOutputCallback(self, callback, user) };

        if r == kCVReturnSuccess {
            Ok(())
        } else {
            Err(r)
        }
    }

    /// Activates a display link.
    #[inline]
    pub fn start(&mut self) -> Result<(), CVReturn> {
        let r = unsafe { CVDisplayLinkStart(self) };

        if r == kCVReturnSuccess {
            Ok(())
        } else {
            Err(r)
        }
    }

    /// Stops a display link.
    #[inline]
    pub fn stop(&mut self) -> Result<(), CVReturn> {
        let r = unsafe { CVDisplayLinkStop(self) };

        if r == kCVReturnSuccess {
            Ok(())
        } else {
            Err(r)
        }
    }

    /// Indicates whether a given display link is running.
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        unsafe { CVDisplayLinkIsRunning(self as *const _ as _) }
    }
}

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "system" {
    unsafe fn CVDisplayLinkCreateWithCGDisplay(
        displayID: CGDirectDisplayID,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
    unsafe fn CVDisplayLinkCreateWithActiveCGDisplays(displayLinkOut: *mut CVDisplayLinkRef) -> CVReturn;
    unsafe fn CVDisplayLinkSetOutputCallback(
        displayLink: CVDisplayLinkRef,
        callback: CVDisplayLinkOutputCallback,
        userInfo: *mut core::ffi::c_void,
    ) -> CVReturn;
    unsafe fn CVDisplayLinkStart(displayLink: CVDisplayLinkRef) -> CVReturn;
    unsafe fn CVDisplayLinkStop(displayLink: CVDisplayLinkRef) -> CVReturn;
    unsafe fn CVDisplayLinkIsRunning(displayLink: CVDisplayLinkRef) -> bool;
    unsafe fn CVDisplayLinkRetain(displayLink: CVDisplayLinkRef) -> CVDisplayLinkRef;
    unsafe fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
}

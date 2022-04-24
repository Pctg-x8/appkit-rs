//! Core Video

use crate::{CGDirectDisplayID, ExternalRc};
use libc::*;
use std::ptr::null_mut;

/// A Core Video error type return value.
pub type CVReturn = i32;
/// Defines a pointer to a display link output callback function, which is called whenever the display link wants
/// the application to output a frame.
pub type CVDisplayLinkOutputCallback = Option<
    extern "system" fn(
        displayLink: CVDisplayLinkRef,
        inNow: *const CVTimeStamp,
        inOutputTime: *const CVTimeStamp,
        flagsIn: CVOptionFlags,
        flagsOut: *mut CVOptionFlags,
        displayLinkContext: *mut c_void,
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
    pub rateScalar: c_double,
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

/// A high-priority thread that notifies your app when a given display will need each frame.
pub enum CVDisplayLink {}
/// A reference to a display link object.
pub type CVDisplayLinkRef = *mut CVDisplayLink;

impl CVDisplayLink {
    /// Creates a display link capable of being used with all active displays.
    pub fn new_for_active_displays() -> Result<ExternalRc<Self>, CVReturn> {
        let mut h = null_mut();
        let r = unsafe { CVDisplayLinkCreateWithActiveCGDisplays(&mut h) };
        if r != 0 {
            Ok(unsafe { ExternalRc::with_fn(h, CVDisplayLinkRetain, CVDisplayLinkRelease) })
        } else {
            Err(r)
        }
    }
    /// Creates a display link for a single display.
    pub fn new_for_display(id: CGDirectDisplayID) -> Result<ExternalRc<Self>, CVReturn> {
        let mut h = null_mut();
        let r = unsafe { CVDisplayLinkCreateWithCGDisplay(id, &mut h) };
        if r == 0 {
            Ok(unsafe { ExternalRc::with_fn(h, CVDisplayLinkRetain, CVDisplayLinkRelease) })
        } else {
            Err(r)
        }
    }

    /// Sets the renderer output callback function
    pub fn set_output_callback(
        &mut self,
        callback: CVDisplayLinkOutputCallback,
        user: *mut c_void,
    ) -> Result<(), CVReturn> {
        let r = unsafe { CVDisplayLinkSetOutputCallback(self, callback, user) };
        if r == 0 {
            Ok(())
        } else {
            Err(r)
        }
    }
    /// Activates a display link.
    pub fn start(&mut self) -> Result<(), CVReturn> {
        let r = unsafe { CVDisplayLinkStart(self) };
        if r == 0 {
            Ok(())
        } else {
            Err(r)
        }
    }
    /// Stops a display link.
    pub fn stop(&mut self) -> Result<(), CVReturn> {
        let r = unsafe { CVDisplayLinkStop(self) };
        if r == 0 {
            Ok(())
        } else {
            Err(r)
        }
    }
    /// Indicates whether a given display link is running.
    pub fn is_running(&self) -> bool {
        unsafe { CVDisplayLinkIsRunning(self as *const _ as _) }
    }
}

#[link(name = "QuartzCore", kind = "framework")]
extern "system" {
    fn CVDisplayLinkCreateWithCGDisplay(
        displayID: CGDirectDisplayID,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
    fn CVDisplayLinkCreateWithActiveCGDisplays(displayLinkOut: *mut CVDisplayLinkRef) -> CVReturn;
    fn CVDisplayLinkSetOutputCallback(
        displayLink: CVDisplayLinkRef,
        callback: CVDisplayLinkOutputCallback,
        userInfo: *mut c_void,
    ) -> CVReturn;
    fn CVDisplayLinkStart(displayLink: CVDisplayLinkRef) -> CVReturn;
    fn CVDisplayLinkStop(displayLink: CVDisplayLinkRef) -> CVReturn;
    fn CVDisplayLinkIsRunning(displayLink: CVDisplayLinkRef) -> bool;
    fn CVDisplayLinkRetain(displayLink: CVDisplayLinkRef) -> CVDisplayLinkRef;
    fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
}

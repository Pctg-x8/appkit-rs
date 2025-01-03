#![allow(non_upper_case_globals)]
//! AudioToolbox

DefineOpaqueFFIObject! {
    pub struct OpaqueAudioComponent;
}

/// An audio component
pub type AudioComponent = *mut OpaqueAudioComponent;

DefineOpaqueFFIObject! {
    pub struct OpaqueAudioComponentInstance;
}

/// An component instance, or object, is an audio unit or audio codec.
pub type AudioComponentInstance = *mut OpaqueAudioComponentInstance;
/// The data type for a plug-in component that provides audio processing or audio data generation.
pub type AudioUnit = AudioComponentInstance;
/// The data type for audio unit property keys.
pub type AudioUnitPropertyID = u32;
/// The data type for audio unit scope identifiers.
pub type AudioUnitScope = u32;
/// The data type for an audio unit element identifier.
pub type AudioUnitElement = u32;

pub const kAudioUnitProperty_StreamFormat: AudioUnitPropertyID = 8;
pub const kAudioUnitProperty_SetRenderCallback: AudioUnitPropertyID = 23;
pub const kAudioUnitScope_Input: AudioUnitScope = 1;
pub const kAudioUnitScope_Output: AudioUnitScope = 2;

pub type AudioFormatID = u32;
pub type AudioFormatFlags = u32;
pub const kAudioFormatFlagIsFloat: AudioFormatFlags = 0x01;
pub const kAudioFormatFlagIsNonInterleaved: AudioFormatFlags = 0x20;

pub type SMPTETimeType = u32;
pub type SMPTETimeFlags = u32;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SMPTETime {
    pub subframes: i16,
    pub subframe_divisor: i16,
    pub counter: u32,
    pub type_: SMPTETimeType,
    pub flags: SMPTETimeFlags,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

pub type AudioTimeStampFlags = u32;
pub type AudioUnitRenderActionFlags = u32;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioTimeStamp {
    pub sample_time: f64,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub word_clock_time: u64,
    pub smpte_time: SMPTETime,
    pub flags: AudioTimeStampFlags,
    pub _reserved: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AudioBufferList {
    pub number_buffers: u32,
    pub buffers: [AudioBuffer; 1], // variadic length
}
#[repr(C)]
#[derive(Debug)]
pub struct AudioBuffer {
    pub number_channels: u32,
    pub data_byte_size: u32,
    pub data: *mut core::ffi::c_void,
}

pub type AURenderCallback = Option<
    extern "C" fn(
        in_ref_con: *mut core::ffi::c_void,
        io_action_flags: *mut AudioUnitRenderActionFlags,
        in_time_stamp: *const AudioTimeStamp,
        in_bus_number: u32,
        in_number_frames: u32,
        io_data: *mut AudioBufferList,
    ) -> super::OSStatus,
>;

#[repr(C)]
#[derive(Debug)]
pub struct AURenderCallbackStruct {
    pub input_proc: AURenderCallback,
    pub input_proc_ref_con: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioComponentDescription {
    pub component_type: super::OSType,
    pub component_subtype: super::OSType,
    pub component_manufacturer: super::OSType,
    pub component_flags: u32,
    pub component_flags_mask: u32,
}

const fn fourcc(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
}

pub const kAudioUnitType_Output: super::OSType = fourcc(*b"auou");
pub const kAudioUnitSubType_HALOutput: super::OSType = fourcc(*b"ahal");
pub const kAudioUnitSubType_DefaultOutput: super::OSType = fourcc(*b"def ");
pub const kAudioUnitSubType_SystemOutput: super::OSType = fourcc(*b"sys ");
pub const kAudioUnitManufacturer_Apple: super::OSType = fourcc(*b"appl");
pub const kAudioFormatLinearPCM: AudioFormatID = fourcc(*b"lpcm");

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioStreamBasicDescription {
    pub sample_rate: f64,
    pub format_id: AudioFormatID,
    pub format_flags: AudioFormatFlags,
    pub bytes_per_packet: u32,
    pub frames_per_packet: u32,
    pub bytes_per_frame: u32,
    pub channels_per_frame: u32,
    pub bits_per_channel: u32,
    pub _reserved: u32,
}

#[link(name = "AudioUnit", kind = "framework")]
extern "system" {
    pub fn AudioComponentFindNext(
        in_component: AudioComponent,
        in_desc: *const AudioComponentDescription,
    ) -> AudioComponent;
    pub fn AudioComponentInstanceNew(
        in_component: AudioComponent,
        out_instance: *mut AudioComponentInstance,
    ) -> super::OSStatus;
    pub fn AudioComponentInstanceDispose(in_instance: AudioComponentInstance) -> super::OSStatus;
    pub fn AudioOutputUnitStart(ci: AudioUnit) -> super::OSStatus;
    pub fn AudioOutputUnitStop(ci: AudioUnit) -> super::OSStatus;

    pub fn AudioUnitInitialize(in_unit: AudioUnit) -> super::OSStatus;
    pub fn AudioUnitUninitialize(in_unit: AudioUnit) -> super::OSStatus;
    pub fn AudioUnitSetProperty(
        in_unit: AudioUnit,
        in_id: AudioUnitPropertyID,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        in_data: *const core::ffi::c_void,
        in_data_size: u32,
    ) -> super::OSStatus;
}

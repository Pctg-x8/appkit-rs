//! AudioToolbox

use libc::c_void;

pub enum OpaqueAudioComponent {}
/// An audio component
pub type AudioComponent = *mut OpaqueAudioComponent;
pub enum OpaqueAudioComponentInstance {}
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

pub type AudioFormatID = u32;
pub type AudioFormatFlags = u32;

pub type SMPTETimeType = u32;
pub type SMPTETimeFlags = u32;
#[repr(C)] #[derive(Debug, Clone)]
pub struct SMPTETime
{
    pub subframes: i16,
    pub subframe_divisor: i16,
    pub counter: u32,
    pub type_: SMPTETimeType,
    pub flags: SMPTETimeFlags,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16
}

pub type AudioTimeStampFlags = u32;
pub type AudioUnitRenderActionFlags = u32;
#[repr(C)] #[derive(Debug, Clone)]
pub struct AudioTimeStamp
{
    pub sample_time: f64,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub word_clock_time: u64,
    pub smpte_time: SMPTETime,
    pub flags: AudioTimeStampFlags,
    pub _reserved: u32
}

#[repr(C)] #[derive(Debug)]
pub struct AudioBufferList
{
    pub number_buffers: u32,
    pub buffers: [AudioBuffer; 1]   // variadic length
}
#[repr(C)] #[derive(Debug)]
pub struct AudioBuffer
{
    pub number_channels: u32,
    pub data_byte_size: u32,
    pub data: *mut c_void
}

pub type AURenderCallback = extern "C" fn(in_ref_con: *mut c_void,
    io_action_flags: *mut AudioUnitRenderActionFlags,
    in_time_stamp: *const AudioTimeStamp,
    in_bus_number: u32,
    in_number_frames: u32,
    io_data: *mut AudioBufferList) -> super::OSStatus;
#[repr(C)] #[derive(Debug)]
pub struct AURenderCallbackStruct
{
    pub input_proc: AURenderCallback,
    pub input_proc_ref_con: *mut c_void
}

#[repr(C)] #[derive(Debug, Clone)]
pub struct AudioComponentDescription
{
    pub component_type: super::OSType,
    pub component_subtype: super::OSType,
    pub component_manufacturer: super::OSType,
    pub component_flags: u32,
    pub component_flags_mask: u32
}

#[repr(C)] #[derive(Debug, Clone)]
pub struct AudioStreamBasicDescription
{
    pub sample_rate: f64,
    pub format_id: AudioFormatID,
    pub format_flags: AudioFormatFlags,
    pub bytes_per_packet: u32,
    pub frames_per_packet: u32,
    pub bytes_per_frame: u32,
    pub channels_per_frame: u32,
    pub bits_per_channel: u32,
    pub _reserved: u32
}

#[link(name="AudioToolbox", kind="framework")]
#[link(name="AudioUnit", kind="framework")]
#[link(name="CoreAudio", kind="framework")]
extern "C"
{
    pub fn AudioComponentFindNext(in_component: AudioComponent, in_desc: *const AudioComponentDescription)
        -> AudioComponent;
    pub fn AudioComponentInstanceNew(in_component: AudioComponent, out_instance: *mut AudioComponentInstance)
        -> super::OSStatus;
    pub fn AudioComponentInstanceDispose(in_instance: AudioComponentInstance) -> super::OSStatus;
    pub fn AudioOutputUnitStart(ci: AudioUnit) -> super::OSStatus;
    pub fn AudioOutputUnitStop(ci: AudioUnit) -> super::OSStatus;

    pub fn AudioUnitInitialize(in_unit: AudioUnit) -> super::OSStatus;
    pub fn AudioUnitUninitialize(in_unit: AudioUnit) -> super::OSStatus;
    pub fn AudioUnitSetProperty(in_unit: AudioUnit, in_id: AudioUnitPropertyID, in_scope: AudioUnitScope,
        in_element: AudioUnitElement, in_data: *const c_void, in_data_size: u32) -> super::OSStatus;
    
    pub static kAudioUnitType_Output: super::OSType;
    pub static kAudioUnitSubType_HALOutput: super::OSType;
    pub static kAudioUnitSubType_DefaultOutput: super::OSType;
    pub static kAudioUnitSubType_SystemOutput: super::OSType;
    pub static kAudioUnitManufacturer_Apple: super::OSType;
    pub static kAudioUnitProperty_StreamFormat: AudioUnitPropertyID;
    pub static kAudioUnitProperty_SetRenderCallback: AudioUnitPropertyID;
    pub static kAudioUnitScope_Input: AudioUnitScope;
    pub static kAudioUnitScope_Output: AudioUnitScope;

    pub static kAudioFormatLinearPCM: AudioFormatID;
    pub static kAudioFormatFlagIsFloat: AudioFormatFlags;
    pub static kAudioFormatFlagIsNonInterleaved: AudioFormatFlags;
}

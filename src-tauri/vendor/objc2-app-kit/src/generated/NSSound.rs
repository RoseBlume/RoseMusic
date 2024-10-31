//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    #[cfg(feature = "NSPasteboard")]
    pub static NSSoundPboardType: &'static NSPasteboardType;
}

pub type NSSoundName = NSString;

pub type NSSoundPlaybackDeviceIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSound;

    unsafe impl ClassType for NSSound {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSSound {}

unsafe impl NSCopying for NSSound {}

unsafe impl NSObjectProtocol for NSSound {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardReading for NSSound {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardWriting for NSSound {}

unsafe impl NSSecureCoding for NSSound {}

extern_methods!(
    unsafe impl NSSound {
        #[method_id(@__retain_semantics Other soundNamed:)]
        pub unsafe fn soundNamed(name: &NSSoundName) -> Option<Retained<NSSound>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:byReference:)]
        pub unsafe fn initWithContentsOfURL_byReference(
            this: Allocated<Self>,
            url: &NSURL,
            by_ref: bool,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:byReference:)]
        pub unsafe fn initWithContentsOfFile_byReference(
            this: Allocated<Self>,
            path: &NSString,
            by_ref: bool,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Option<Retained<Self>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&NSSoundName>) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSSoundName>>;

        #[cfg(feature = "NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[method_id(@__retain_semantics Other soundUnfilteredTypes)]
        pub unsafe fn soundUnfilteredTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            this: Allocated<Self>,
            pasteboard: &NSPasteboard,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteboard: &NSPasteboard);

        #[method(play)]
        pub unsafe fn play(&self) -> bool;

        #[method(pause)]
        pub unsafe fn pause(&self) -> bool;

        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;

        #[method(stop)]
        pub unsafe fn stop(&self) -> bool;

        #[method(isPlaying)]
        pub unsafe fn isPlaying(&self) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn NSSoundDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSSoundDelegate>>);

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> NSTimeInterval;

        #[method(setCurrentTime:)]
        pub unsafe fn setCurrentTime(&self, current_time: NSTimeInterval);

        #[method(loops)]
        pub unsafe fn loops(&self) -> bool;

        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);

        #[method_id(@__retain_semantics Other playbackDeviceIdentifier)]
        pub unsafe fn playbackDeviceIdentifier(
            &self,
        ) -> Option<Retained<NSSoundPlaybackDeviceIdentifier>>;

        #[method(setPlaybackDeviceIdentifier:)]
        pub unsafe fn setPlaybackDeviceIdentifier(
            &self,
            playback_device_identifier: Option<&NSSoundPlaybackDeviceIdentifier>,
        );

        #[deprecated]
        #[method(setChannelMapping:)]
        pub unsafe fn setChannelMapping(&self, channel_mapping: Option<&NSArray>);

        #[deprecated]
        #[method_id(@__retain_semantics Other channelMapping)]
        pub unsafe fn channelMapping(&self) -> Option<Retained<NSArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSound {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSSound {
        #[deprecated]
        #[method_id(@__retain_semantics Other soundUnfilteredFileTypes)]
        pub unsafe fn soundUnfilteredFileTypes() -> Option<Retained<NSArray>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other soundUnfilteredPasteboardTypes)]
        pub unsafe fn soundUnfilteredPasteboardTypes() -> Option<Retained<NSArray>>;
    }
);

extern_protocol!(
    pub unsafe trait NSSoundDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(sound:didFinishPlaying:)]
        unsafe fn sound_didFinishPlaying(&self, sound: &NSSound, flag: bool);
    }

    unsafe impl ProtocolType for dyn NSSoundDelegate {}
);

extern_category!(
    /// Category on [`NSBundle`].
    pub unsafe trait NSBundleSoundExtensions {
        #[method_id(@__retain_semantics Other pathForSoundResource:)]
        unsafe fn pathForSoundResource(&self, name: &NSSoundName) -> Option<Retained<NSString>>;
    }

    unsafe impl NSBundleSoundExtensions for NSBundle {}
);

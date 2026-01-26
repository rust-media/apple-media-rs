use std::ptr::null_mut;

use cfg_if::cfg_if;
use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, OSStatus, TCFType},
    string::{CFString, CFStringEncoding, CFStringRef},
};
use libc::size_t;

use crate::{
    block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    format_description::{
        CMAudioFormatDescription, CMAudioFormatDescriptionRef, CMClosedCaptionFormatDescriptionRef, CMMediaType, CMMetadataFormatDescriptionRef,
        CMTextFormatDescriptionRef, CMTimeCodeFormatDescriptionRef, CMVideoFormatDescription, CMVideoFormatDescriptionRef,
    },
};

pub const kCMFormatDescriptionBridgeError_InvalidParameter: OSStatus = -12712;
pub const kCMFormatDescriptionBridgeError_AllocationFailed: OSStatus = -12713;
pub const kCMFormatDescriptionBridgeError_InvalidSerializedSampleDescription: OSStatus = -12714;
pub const kCMFormatDescriptionBridgeError_InvalidFormatDescription: OSStatus = -12715;
pub const kCMFormatDescriptionBridgeError_IncompatibleFormatDescription: OSStatus = -12716;
pub const kCMFormatDescriptionBridgeError_UnsupportedSampleDescriptionFlavor: OSStatus = -12717;
pub const kCMFormatDescriptionBridgeError_InvalidSlice: OSStatus = -12719;

extern "C" {
    pub static kCMImageDescriptionFlavor_QuickTimeMovie: CFStringRef;
    pub static kCMImageDescriptionFlavor_ISOFamily: CFStringRef;
    pub static kCMImageDescriptionFlavor_3GPFamily: CFStringRef;
    pub static kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions: CFStringRef;

    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
        allocator: CFAllocatorRef,
        imageDescriptionData: *const u8,
        size: size_t,
        stringEncoding: CFStringEncoding,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        imageDescriptionBlockBuffer: CMBlockBufferRef,
        stringEncoding: CFStringEncoding,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        videoFormatDescription: CMVideoFormatDescriptionRef,
        stringEncoding: CFStringEncoding,
        flavor: CFStringRef,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;

    cfg_if! {
        if #[cfg(target_endian = "little")] {
            pub fn CMSwapBigEndianImageDescriptionToHost(imageDescriptionData: *mut u8, imageDescriptionSize: size_t) -> OSStatus;
            pub fn CMSwapHostEndianImageDescriptionToBig(imageDescriptionData: *mut u8, imageDescriptionSize: size_t) -> OSStatus;
        }
    }

    pub static kCMSoundDescriptionFlavor_QuickTimeMovie: CFStringRef;
    pub static kCMSoundDescriptionFlavor_QuickTimeMovieV2: CFStringRef;
    pub static kCMSoundDescriptionFlavor_ISOFamily: CFStringRef;
    pub static kCMSoundDescriptionFlavor_3GPFamily: CFStringRef;

    pub fn CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData(
        allocator: CFAllocatorRef,
        soundDescriptionData: *const u8,
        size: size_t,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        soundDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        audioFormatDescription: CMAudioFormatDescriptionRef,
        flavor: CFStringRef,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
    pub fn CMDoesBigEndianSoundDescriptionRequireLegacyCBRSampleTableLayout(
        soundDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CFStringRef,
    ) -> Boolean;

    cfg_if! {
        if #[cfg(target_endian = "little")] {
            pub fn CMSwapBigEndianSoundDescriptionToHost(soundDescriptionData: *mut u8, soundDescriptionSize: size_t) -> OSStatus;
            pub fn CMSwapHostEndianSoundDescriptionToBig(soundDescriptionData: *mut u8, soundDescriptionSize: size_t) -> OSStatus;
        }
    }

    pub fn CMTextFormatDescriptionCreateFromBigEndianTextDescriptionData(
        allocator: CFAllocatorRef,
        textDescriptionData: *const u8,
        size: size_t,
        flavor: CFStringRef,
        mediaType: CMMediaType,
        formatDescriptionOut: *mut CMTextFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMTextFormatDescriptionCreateFromBigEndianTextDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        textDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CFStringRef,
        mediaType: CMMediaType,
        formatDescriptionOut: *mut CMTextFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMTextFormatDescriptionCopyAsBigEndianTextDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        textFormatDescription: CMTextFormatDescriptionRef,
        flavor: CFStringRef,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;

    cfg_if! {
        if #[cfg(target_endian = "little")] {
            pub fn CMSwapBigEndianTextDescriptionToHost(textDescriptionData: *mut u8, textDescriptionSize: size_t) -> OSStatus;
            pub fn CMSwapHostEndianTextDescriptionToBig(textDescriptionData: *mut u8, textDescriptionSize: size_t) -> OSStatus;
        }
    }

    pub fn CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionData(
        allocator: CFAllocatorRef,
        closedCaptionDescriptionData: *const u8,
        size: size_t,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMClosedCaptionFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        closedCaptionDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMClosedCaptionFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMClosedCaptionFormatDescriptionCopyAsBigEndianClosedCaptionDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        closedCaptionFormatDescription: CMClosedCaptionFormatDescriptionRef,
        flavor: CFStringRef,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;

    cfg_if! {
        if #[cfg(target_endian = "little")] {
            pub fn CMSwapBigEndianClosedCaptionDescriptionToHost(closedCaptionDescriptionData: *mut u8, closedCaptionDescriptionSize: size_t) -> OSStatus;
            pub fn CMSwapHostEndianClosedCaptionDescriptionToBig(closedCaptionDescriptionData: *mut u8, closedCaptionDescriptionSize: size_t) -> OSStatus;
        }
    }

    pub fn CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionData(
        allocator: CFAllocatorRef,
        timeCodeDescriptionData: *const u8,
        size: size_t,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMTimeCodeFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        timeCodeDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMTimeCodeFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMTimeCodeFormatDescriptionCopyAsBigEndianTimeCodeDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        timeCodeFormatDescription: CMTimeCodeFormatDescriptionRef,
        flavor: CFStringRef,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;

    cfg_if! {
        if #[cfg(target_endian = "little")] {
            pub fn CMSwapBigEndianTimeCodeDescriptionToHost(timeCodeDescriptionData: *mut u8, timeCodeDescriptionSize: size_t) -> OSStatus;
            pub fn CMSwapHostEndianTimeCodeDescriptionToBig(timeCodeDescriptionData: *mut u8, timeCodeDescriptionSize: size_t) -> OSStatus;
        }
    }

    pub fn CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionData(
        allocator: CFAllocatorRef,
        metadataDescriptionData: *const u8,
        size: size_t,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        metadataDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CFStringRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMMetadataFormatDescriptionCopyAsBigEndianMetadataDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        metadataFormatDescription: CMMetadataFormatDescriptionRef,
        flavor: CFStringRef,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;

    cfg_if! {
        if #[cfg(target_endian = "little")] {
            pub fn CMSwapBigEndianMetadataDescriptionToHost(metadataDescriptionData: *mut u8, metadataDescriptionSize: size_t) -> OSStatus;
            pub fn CMSwapHostEndianMetadataDescriptionToBig(metadataDescriptionData: *mut u8, metadataDescriptionSize: size_t) -> OSStatus;
        }
    }
}

impl CMAudioFormatDescription {
    #[inline]
    pub fn from_big_endian_sound_description_data(sound_descriptionData: &[u8], flavor: &CFString) -> Result<CMAudioFormatDescription, OSStatus> {
        let mut format_description: CMAudioFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData(
                kCFAllocatorDefault,
                sound_descriptionData.as_ptr(),
                sound_descriptionData.len(),
                flavor.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { CMAudioFormatDescription::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn from_big_endian_sound_description_block_buffer(
        sound_description_block_buffer: &CMBlockBuffer,
        flavor: &CFString,
    ) -> Result<CMAudioFormatDescription, OSStatus> {
        let mut format_description: CMAudioFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer(
                kCFAllocatorDefault,
                sound_description_block_buffer.as_concrete_TypeRef(),
                flavor.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { CMAudioFormatDescription::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn copy_as_big_endian_sound_description_block_buffer(&self, flavor: &CFString) -> Result<CMBlockBuffer, OSStatus> {
        let mut block_buffer: CMBlockBufferRef = null_mut();
        let status = unsafe {
            CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                flavor.as_concrete_TypeRef(),
                &mut block_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMBlockBuffer::wrap_under_create_rule(block_buffer) })
        } else {
            Err(status)
        }
    }
}

impl CMBlockBuffer {
    #[inline]
    pub fn does_big_endian_sound_description_require_legacy_cbr_sample_table_layout(&self, flavor: &CFString) -> bool {
        unsafe { CMDoesBigEndianSoundDescriptionRequireLegacyCBRSampleTableLayout(self.as_concrete_TypeRef(), flavor.as_concrete_TypeRef()) != 0 }
    }
}

impl CMVideoFormatDescription {
    #[inline]
    pub fn from_big_endian_image_description_data(
        image_description_data: &[u8],
        string_encoding: CFStringEncoding,
        flavor: &CFString,
    ) -> Result<CMVideoFormatDescription, OSStatus> {
        let mut format_description: CMVideoFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
                kCFAllocatorDefault,
                image_description_data.as_ptr(),
                image_description_data.len(),
                string_encoding,
                flavor.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { CMVideoFormatDescription::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn from_big_endian_image_description_block_buffer(
        image_description_block_buffer: &CMBlockBuffer,
        string_encoding: CFStringEncoding,
        flavor: &CFString,
    ) -> Result<CMVideoFormatDescription, OSStatus> {
        let mut format_description: CMVideoFormatDescriptionRef = null_mut();
        let status = unsafe {
            CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
                kCFAllocatorDefault,
                image_description_block_buffer.as_concrete_TypeRef(),
                string_encoding,
                flavor.as_concrete_TypeRef(),
                &mut format_description,
            )
        };
        if status == 0 {
            Ok(unsafe { CMVideoFormatDescription::wrap_under_create_rule(format_description) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn copy_as_big_endian_image_description_block_buffer(
        &self,
        string_encoding: CFStringEncoding,
        flavor: &CFString,
    ) -> Result<CMBlockBuffer, OSStatus> {
        let mut block_buffer: CMBlockBufferRef = null_mut();
        let status = unsafe {
            CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                string_encoding,
                flavor.as_concrete_TypeRef(),
                &mut block_buffer,
            )
        };
        if status == 0 {
            Ok(unsafe { CMBlockBuffer::wrap_under_create_rule(block_buffer) })
        } else {
            Err(status)
        }
    }
}

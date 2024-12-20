extern crate core_foundation;
extern crate video_toolbox_sys;

use core_foundation::array::{CFArray, CFArrayCreate, CFArrayRef};
use core_foundation::base::{kCFAllocatorDefault, CFIndexConvertible, TCFType};
use core_foundation::dictionary::{
    kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionary,
    CFDictionaryCreate, CFDictionaryRef,
};
use core_foundation::string::CFString;

use video_toolbox_sys::utilities::{
    kVTVideoEncoderList_CodecName, kVTVideoEncoderList_CodecType, kVTVideoEncoderList_DisplayName,
    kVTVideoEncoderList_EncoderID, kVTVideoEncoderList_EncoderName, VTCopyVideoEncoderList,
};

use std::mem;
use std::ptr;

unsafe fn run() {
    println!(
        "kVTVideoEncoderList_CodecType: {:?}",
        CFString::wrap_under_create_rule(kVTVideoEncoderList_CodecType)
    );
    println!(
        "kVTVideoEncoderList_EncoderID: {:?}",
        CFString::wrap_under_create_rule(kVTVideoEncoderList_EncoderID)
    );
    println!(
        "kVTVideoEncoderList_CodecName: {:?}",
        CFString::wrap_under_create_rule(kVTVideoEncoderList_CodecName)
    );
    println!(
        "kVTVideoEncoderList_EncoderName: {:?}",
        CFString::wrap_under_create_rule(kVTVideoEncoderList_EncoderName)
    );
    println!(
        "kVTVideoEncoderList_DisplayName: {:?}",
        CFString::wrap_under_create_rule(kVTVideoEncoderList_DisplayName)
    );
    println!("\n\n\n");

    let keys: Vec<CFString> = vec![
        CFString::new("CodecName"),
        CFString::new("CodecType"),
        CFString::new("EncoderID"),
        CFString::new("EncoderName"),
        CFString::new("DisplayName"),
    ];

    let values: Vec<CFString> = vec![
        CFString::new(""),
        CFString::new(""),
        CFString::new(""),
        CFString::new(""),
        CFString::new(""),
    ];

    let opts_ref: CFDictionaryRef = CFDictionaryCreate(
        kCFAllocatorDefault,
        mem::transmute(keys.as_ptr()),
        mem::transmute(values.as_ptr()),
        keys.len().to_CFIndex(),
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );

    let mut result_ref: CFArrayRef = CFArrayCreate(
        kCFAllocatorDefault,
        ptr::null_mut(),
        0.to_CFIndex(),
        ptr::null(),
    );

    let ret_code = VTCopyVideoEncoderList(opts_ref, &mut result_ref);

    println!(
        "opts: {:?}",
        CFDictionary::<CFString, CFString>::wrap_under_create_rule(opts_ref)
    );
    println!("ret_code: {:?}", ret_code);
    println!(
        "result: {:?}",
        CFArray::<CFString>::wrap_under_create_rule(result_ref)
    );
}

fn main() {
    unsafe {
        run();
    }
}

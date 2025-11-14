// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use libspa::pod::Pod;
use std::ffi::CStr;

/// Read a `Pod`'s string if it contains a string.
pub fn string_from_pod(pod: &Pod) -> Option<String> {
    if !pod.is_string() {
        return None;
    }

    let mut cstr = std::ptr::null();

    unsafe {
        // SAFETY: Pod is checked to be a string beforehand
        if libspa_sys::spa_pod_get_string(pod.as_raw_ptr(), &mut cstr) == 0 {
            if !cstr.is_null() {
                return Some(String::from_utf8_lossy(CStr::from_ptr(cstr).to_bytes()).into_owned());
            }
        }
    }

    None
}

/// SAFETY: Must be absolutely certain that the array is a compatible array.
pub unsafe fn array_from_pod<CType: Copy>(pod: &Pod) -> Option<Vec<CType>> {
    if !pod.is_array() {
        return None;
    }

    let mut len = 0;

    unsafe {
        let array: *mut CType = libspa_sys::spa_pod_get_array(pod.as_raw_ptr(), &mut len).cast();

        if array.is_null() {
            return None;
        }

        Some(std::slice::from_raw_parts(array, len as usize).to_vec())
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum Channel {
    #[default]
    UNKNOWN = 0, // unspecified
    NA,          // N/A, silent
    MONO,        // mono stream
    FL,          // front left
    FR,          // front right
    FC,          // front center
    LFE,         // LFE
    SL,          // side left
    SR,          // side right
    FLC,         // front left center
    FRC,         // front right center
    RC,          // rear center
    RL,          // rear left
    RR,          // rear right
    TC,          // top center
    TFL,         // top front left
    TFC,         // top front center
    TFR,         // top front right
    TRL,         // top rear left
    TRC,         // top rear center
    TRR,         // top rear right
    RLC,         // rear left center
    RRC,         // rear right center
    FLW,         // front left wide
    FRW,         // front right wide
    LFE2,        // LFE 2
    FLH,         // front left high
    FCH,         // front center high
    FRH,         // front right high
    TFLC,        // top front left center
    TFRC,        // top front right center
    TSL,         // top side left
    TSR,         // top side right
    LLFE,        // left LFE
    RLFE,        // right LFE
    BC,          // bottom center
    BLC,         // bottom left center
    BRC = 37,    // bottom right center
    AUX0 = 4096, // aux channels
    AUX1,
    AUX2,
    AUX3,
    AUX4,
    AUX5,
    AUX6,
    AUX7,
    AUX8,
    AUX9,
    AUX10,
    AUX11,
    AUX12,
    AUX13,
    AUX14,
    AUX15,
    AUX16,
    AUX17,
    AUX18,
    AUX19,
    AUX20,
    AUX21,
    AUX22,
    AUX23,
    AUX24,
    AUX25,
    AUX26,
    AUX27,
    AUX28,
    AUX29,
    AUX30,
    AUX31,
    AUX32,
    AUX33,
    AUX34,
    AUX35,
    AUX36,
    AUX37,
    AUX38,
    AUX39,
    AUX40,
    AUX41,
    AUX42,
    AUX43,
    AUX44,
    AUX45,
    AUX46,
    AUX47,
    AUX48,
    AUX49,
    AUX50,
    AUX51,
    AUX52,
    AUX53,
    AUX54,
    AUX55,
    AUX56,
    AUX57,
    AUX58,
    AUX59,
    AUX60,
    AUX61,
    AUX62,
    AUX63 = 4159,
}

// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use std::mem;

use glib::translate::*;
use gst_sdp_sys;

#[repr(C)]
pub struct SDPBandwidth(pub(crate) gst_sdp_sys::GstSDPBandwidth);

unsafe impl Send for SDPBandwidth {}
unsafe impl Sync for SDPBandwidth {}

impl SDPBandwidth {
    pub fn new(bwtype: &str, bandwidth: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut bw = mem::MaybeUninit::zeroed();
            gst_sdp_sys::gst_sdp_bandwidth_set(bw.as_mut_ptr(), bwtype.to_glib_none().0, bandwidth);
            SDPBandwidth(bw.assume_init())
        }
    }

    pub fn bwtype(&self) -> Option<&str> {
        unsafe {
            if self.0.bwtype.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.bwtype).to_str().unwrap())
            }
        }
    }

    pub fn value(&self) -> u32 {
        self.0.bandwidth as u32
    }
}

impl Clone for SDPBandwidth {
    fn clone(&self) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut bw = mem::MaybeUninit::zeroed();
            gst_sdp_sys::gst_sdp_bandwidth_set(bw.as_mut_ptr(), self.0.bwtype, self.0.bandwidth);
            SDPBandwidth(bw.assume_init())
        }
    }
}

impl Drop for SDPBandwidth {
    fn drop(&mut self) {
        unsafe {
            gst_sdp_sys::gst_sdp_bandwidth_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPBandwidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPBandwidth")
            .field("bwtype", &self.bwtype())
            .field("value", &self.value())
            .finish()
    }
}

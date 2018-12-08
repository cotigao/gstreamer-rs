// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EncodingProfile;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct EncodingVideoProfile(Object<ffi::GstEncodingVideoProfile, ffi::GstEncodingVideoProfileClass>): EncodingProfile;

    match fn {
        get_type => || ffi::gst_encoding_video_profile_get_type(),
    }
}

impl EncodingVideoProfile {
    pub fn get_pass(&self) -> u32 {
        unsafe {
            ffi::gst_encoding_video_profile_get_pass(self.to_glib_none().0)
        }
    }

    pub fn get_variableframerate(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_video_profile_get_variableframerate(self.to_glib_none().0))
        }
    }
}

unsafe impl Send for EncodingVideoProfile {}
unsafe impl Sync for EncodingVideoProfile {}
// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use DesktopAppInfo;

impl DesktopAppInfo {
    pub fn search(search_string: &str) -> Vec<Vec<GString>> {
        unsafe {
            let out = gio_sys::g_desktop_app_info_search(search_string.to_glib_none().0);

            if out.is_null() {
                return Vec::new();
            }

            let mut ret = Vec::new();
            let mut it = 0;
            loop {
                let tmp: *mut *mut libc::c_char = *out.offset(it);

                if tmp.is_null() {
                    break;
                }
                let v: Vec<GString> = FromGlibPtrContainer::from_glib_full(tmp);
                ret.push(v);
                it += 1;
            }

            glib_sys::g_free(out as *mut libc::c_void);
            ret
        }
    }
}

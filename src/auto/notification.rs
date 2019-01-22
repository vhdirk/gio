// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_40", feature = "dox"))]
use Icon;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use NotificationPriority;
use ffi;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Notification(Object<ffi::GNotification, NotificationClass>);

    match fn {
        get_type => || ffi::g_notification_get_type(),
    }
}

impl Notification {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn new(title: &str) -> Notification {
        unsafe {
            from_glib_full(ffi::g_notification_new(title.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn add_button(&self, label: &str, detailed_action: &str) {
        unsafe {
            ffi::g_notification_add_button(self.to_glib_none().0, label.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //pub fn add_button_with_target<'a, P: Into<Option<&'a str>>>(&self, label: &str, action: &str, target_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_notification_add_button_with_target() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn add_button_with_target_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, label: &str, action: &str, target: P) {
        let target = target.into();
        unsafe {
            ffi::g_notification_add_button_with_target_value(self.to_glib_none().0, label.to_glib_none().0, action.to_glib_none().0, target.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn set_body<'a, P: Into<Option<&'a str>>>(&self, body: P) {
        let body = body.into();
        unsafe {
            ffi::g_notification_set_body(self.to_glib_none().0, body.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn set_default_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_notification_set_default_action(self.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //pub fn set_default_action_and_target<'a, P: Into<Option<&'a str>>>(&self, action: &str, target_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_notification_set_default_action_and_target() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn set_default_action_and_target_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, action: &str, target: P) {
        let target = target.into();
        unsafe {
            ffi::g_notification_set_default_action_and_target_value(self.to_glib_none().0, action.to_glib_none().0, target.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn set_icon<P: IsA<Icon>>(&self, icon: &P) {
        unsafe {
            ffi::g_notification_set_icon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    pub fn set_priority(&self, priority: NotificationPriority) {
        unsafe {
            ffi::g_notification_set_priority(self.to_glib_none().0, priority.to_glib());
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::g_notification_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v2_42", deprecated)]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn set_urgent(&self, urgent: bool) {
        unsafe {
            ffi::g_notification_set_urgent(self.to_glib_none().0, urgent.to_glib());
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notification")
    }
}

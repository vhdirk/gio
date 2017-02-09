// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use MenuItem;
use MenuModel;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Menu(Object<ffi::GMenu>): MenuModel;

    match fn {
        get_type => || ffi::g_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        unsafe {
            from_glib_full(ffi::g_menu_new())
        }
    }

    pub fn append<'a, T: Into<Option<&'a str>>, U: Into<Option<&'a str>>>(&self, label: T, detailed_action: U) {
        unsafe {
            ffi::g_menu_append(self.to_glib_none().0, label.into().to_glib_none().0, detailed_action.into().to_glib_none().0);
        }
    }

    pub fn append_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_append_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    pub fn append_section<'a, T: Into<Option<&'a str>>, U: IsA<MenuModel>>(&self, label: T, section: &U) {
        unsafe {
            ffi::g_menu_append_section(self.to_glib_none().0, label.into().to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn append_submenu<'a, T: Into<Option<&'a str>>, U: IsA<MenuModel>>(&self, label: T, submenu: &U) {
        unsafe {
            ffi::g_menu_append_submenu(self.to_glib_none().0, label.into().to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    pub fn freeze(&self) {
        unsafe {
            ffi::g_menu_freeze(self.to_glib_none().0);
        }
    }

    pub fn insert<'a, T: Into<Option<&'a str>>, U: Into<Option<&'a str>>>(&self, position: i32, label: T, detailed_action: U) {
        unsafe {
            ffi::g_menu_insert(self.to_glib_none().0, position, label.into().to_glib_none().0, detailed_action.into().to_glib_none().0);
        }
    }

    pub fn insert_item(&self, position: i32, item: &MenuItem) {
        unsafe {
            ffi::g_menu_insert_item(self.to_glib_none().0, position, item.to_glib_none().0);
        }
    }

    pub fn insert_section<'a, T: Into<Option<&'a str>>, U: IsA<MenuModel>>(&self, position: i32, label: T, section: &U) {
        unsafe {
            ffi::g_menu_insert_section(self.to_glib_none().0, position, label.into().to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn insert_submenu<'a, T: Into<Option<&'a str>>, U: IsA<MenuModel>>(&self, position: i32, label: T, submenu: &U) {
        unsafe {
            ffi::g_menu_insert_submenu(self.to_glib_none().0, position, label.into().to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    pub fn prepend<'a, T: Into<Option<&'a str>>, U: Into<Option<&'a str>>>(&self, label: T, detailed_action: U) {
        unsafe {
            ffi::g_menu_prepend(self.to_glib_none().0, label.into().to_glib_none().0, detailed_action.into().to_glib_none().0);
        }
    }

    pub fn prepend_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_prepend_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    pub fn prepend_section<'a, T: Into<Option<&'a str>>, U: IsA<MenuModel>>(&self, label: T, section: &U) {
        unsafe {
            ffi::g_menu_prepend_section(self.to_glib_none().0, label.into().to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn prepend_submenu<'a, T: Into<Option<&'a str>>, U: IsA<MenuModel>>(&self, label: T, submenu: &U) {
        unsafe {
            ffi::g_menu_prepend_submenu(self.to_glib_none().0, label.into().to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe {
            ffi::g_menu_remove(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v2_38")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_menu_remove_all(self.to_glib_none().0);
        }
    }
}

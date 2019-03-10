// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use File;
use InputStream;
use gio_sys;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct ApplicationCommandLine(Object<gio_sys::GApplicationCommandLine, gio_sys::GApplicationCommandLineClass, ApplicationCommandLineClass>);

    match fn {
        get_type => || gio_sys::g_application_command_line_get_type(),
    }
}

pub const NONE_APPLICATION_COMMAND_LINE: Option<&ApplicationCommandLine> = None;

pub trait ApplicationCommandLineExt: 'static {
    fn create_file_for_arg<P: AsRef<std::ffi::OsStr>>(&self, arg: P) -> Option<File>;

    fn get_arguments(&self) -> Vec<std::ffi::OsString>;

    fn get_cwd(&self) -> Option<std::path::PathBuf>;

    fn get_environ(&self) -> Vec<std::ffi::OsString>;

    fn get_exit_status(&self) -> i32;

    fn get_is_remote(&self) -> bool;

    //fn get_options_dict(&self) -> /*Ignored*/Option<glib::VariantDict>;

    fn get_platform_data(&self) -> Option<glib::Variant>;

    fn get_stdin(&self) -> Option<InputStream>;

    fn getenv<P: AsRef<std::ffi::OsStr>>(&self, name: P) -> Option<GString>;

    //fn print(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn printerr(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_exit_status(&self, exit_status: i32);

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ApplicationCommandLine>> ApplicationCommandLineExt for O {
    fn create_file_for_arg<P: AsRef<std::ffi::OsStr>>(&self, arg: P) -> Option<File> {
        unsafe {
            from_glib_full(gio_sys::g_application_command_line_create_file_for_arg(self.as_ref().to_glib_none().0, arg.as_ref().to_glib_none().0))
        }
    }

    fn get_arguments(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            let mut argc = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(gio_sys::g_application_command_line_get_arguments(self.as_ref().to_glib_none().0, &mut argc), argc as usize);
            ret
        }
    }

    fn get_cwd(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(gio_sys::g_application_command_line_get_cwd(self.as_ref().to_glib_none().0))
        }
    }

    fn get_environ(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gio_sys::g_application_command_line_get_environ(self.as_ref().to_glib_none().0))
        }
    }

    fn get_exit_status(&self) -> i32 {
        unsafe {
            gio_sys::g_application_command_line_get_exit_status(self.as_ref().to_glib_none().0)
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_application_command_line_get_is_remote(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_options_dict(&self) -> /*Ignored*/Option<glib::VariantDict> {
    //    unsafe { TODO: call gio_sys:g_application_command_line_get_options_dict() }
    //}

    fn get_platform_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(gio_sys::g_application_command_line_get_platform_data(self.as_ref().to_glib_none().0))
        }
    }

    fn get_stdin(&self) -> Option<InputStream> {
        unsafe {
            from_glib_full(gio_sys::g_application_command_line_get_stdin(self.as_ref().to_glib_none().0))
        }
    }

    fn getenv<P: AsRef<std::ffi::OsStr>>(&self, name: P) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_application_command_line_getenv(self.as_ref().to_glib_none().0, name.as_ref().to_glib_none().0))
        }
    }

    //fn print(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gio_sys:g_application_command_line_print() }
    //}

    //fn printerr(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gio_sys:g_application_command_line_printerr() }
    //}

    fn set_exit_status(&self, exit_status: i32) {
        unsafe {
            gio_sys::g_application_command_line_set_exit_status(self.as_ref().to_glib_none().0, exit_status);
        }
    }

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-remote\0".as_ptr() as *const _,
                Some(transmute(notify_is_remote_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_is_remote_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GApplicationCommandLine, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ApplicationCommandLine> {
    let f: &F = &*(f as *const F);
    f(&ApplicationCommandLine::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ApplicationCommandLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApplicationCommandLine")
    }
}

use std::convert;
use std::ffi::OsString;
use std::fmt;
use std::ops;
use std::ptr;

use libc;

use ffi;
use glib_ffi;

use super::prelude::*;
use glib;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use Application;
use ApplicationClass;
use File;

pub struct ArgumentList {
    pub(crate) ptr: *mut *mut *mut libc::c_char,
    items: Vec<OsString>,
}

impl ArgumentList {
    pub(crate) fn new(arguments: *mut *mut *mut libc::c_char) -> Self {
        Self {
            ptr: arguments,
            items: unsafe { FromGlibPtrContainer::from_glib_none(ptr::read(arguments)) },
        }
    }

    pub(crate) fn refresh(&mut self) {
        self.items = unsafe { FromGlibPtrContainer::from_glib_none(ptr::read(self.ptr)) };
    }

    // remove the item at index `idx` and shift the raw array
    pub fn remove(&mut self, idx: usize) {
        unsafe {
            let n_args = glib_ffi::g_strv_length(*self.ptr);
            assert!((n_args as usize) == self.items.len());
            assert!((idx as u32) < n_args);

            self.items.remove(idx);

            glib_ffi::g_free(((*self.ptr).offset(idx as isize)) as *mut libc::c_void);

            for i in (idx as u32)..n_args - 1 {
                ptr::write(
                    (*self.ptr).offset(i as isize),
                    *(*self.ptr).offset((i + 1) as isize),
                )
            }
            ptr::write((*self.ptr).offset((n_args - 1) as isize), ptr::null_mut());
        }
    }
}

impl ops::Deref for ArgumentList {
    type Target = [OsString];

    fn deref(&self) -> &Self::Target {
        self.items.as_slice()
    }
}

impl fmt::Debug for ArgumentList {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.items.fmt(formatter)
    }
}

impl convert::Into<Vec<OsString>> for ArgumentList {
    fn into(self) -> Vec<OsString> {
        self.items.clone()
    }
}


pub trait ApplicationImpl: ObjectImpl + Send + Sync + 'static {

    fn activate(&self, application: &::Application){
        self.parent_activate(application)
    }

    fn after_emit(&self, application: &::Application, platform_data: &glib::Variant){
        self.parent_after_emit(application, platform_data)
    }

    fn before_emit(&self, application: &::Application, platform_data: &glib::Variant){
        self.parent_before_emit(application, platform_data)
    }

    fn command_line(&self, application: &::Application, command_line: /*Ignored*/&::ApplicationCommandLine) -> i32{
        self.parent_command_line(application, command_line)
    }

    fn local_command_line(&self, application: &::Application, arguments: &mut ArgumentList) -> Option<i32> {
        self.parent_local_command_line(application, arguments)
    }

    fn open(&self, application: &::Application, files: /*Ignored*/&[::File], hint: &str){
        self.parent_open(application, files, hint)
    }

    fn quit_mainloop(&self, application: &::Application){
        self.parent_quit_mainloop(application)
    }

    fn run_mainloop(&self, application: &::Application){
        self.parent_run_mainloop(application)
    }

    fn shutdown(&self, application: &::Application){
        self.parent_shutdown(application)
    }

    fn startup(&self, application: &::Application){
        self.parent_startup(application)
    }

    fn parent_activate(&self, application: &::Application){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .activate
            .map(|f|{ f(application.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_after_emit(&self, application: &::Application, platform_data: &glib::Variant){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .after_emit
            .map(|f|{ f(application.to_glib_none().0,platform_data.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_before_emit(&self, application: &::Application, platform_data: &glib::Variant){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .before_emit
            .map(|f|{ f(application.to_glib_none().0,platform_data.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_command_line(&self, application: &::Application, cmd_line: &::ApplicationCommandLine) -> i32 {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
                .command_line
                .map(|f| f(application.to_glib_none().0, cmd_line.to_glib_none().0))
                .unwrap_or(0)
        }
    }

    fn parent_local_command_line(&self, application: &::Application, arguments: &mut ArgumentList) -> Option<i32> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let mut exit_status = 0;
            let success = (*parent_class)
                .local_command_line
                .map(|f| {
                    let ret = f(application.to_glib_none().0, arguments.ptr, &mut exit_status);
                    arguments.refresh();
                    ret
                })
                .unwrap_or(glib_ffi::GFALSE);

            match success {
                glib_ffi::GTRUE => Some(exit_status),
                _ => None,
            }
        }
    }

    fn parent_open(&self, application: &::Application, files: /*Ignored*/&[::File], hint: &str){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .open
            .map(|f|{ let n_files = files.len() as i32; f(application.to_glib_none().0,files.to_glib_none().0,n_files,hint.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_quit_mainloop(&self, application: &::Application){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .quit_mainloop
            .map(|f|{ f(application.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_run_mainloop(&self, application: &::Application){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .run_mainloop
            .map(|f|{ f(application.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_shutdown(&self, application: &::Application){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .shutdown
            .map(|f|{ f(application.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_startup(&self, application: &::Application){
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            (*parent_class)
            .startup
            .map(|f|{ f(application.to_glib_none().0); })
            .unwrap_or(())
        }
    }

}

pub unsafe trait ApplicationClassSubclassExt: Sized + 'static {}

unsafe impl ApplicationClassSubclassExt for ApplicationClass {}

unsafe impl<T: ObjectSubclass + ApplicationImpl> IsSubclassable<T> for ApplicationClass
{
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);

        unsafe {
            let klass = &mut *(self as *const Self as *mut ffi::GApplicationClass);
            klass.activate = Some(application_activate::<T>);
            klass.after_emit = Some(application_after_emit::<T>);
            klass.before_emit = Some(application_before_emit::<T>);
            klass.command_line = Some(application_command_line::<T>);
            klass.local_command_line = Some(application_local_command_line::<T>);
            klass.open = Some(application_open::<T>);
            klass.quit_mainloop = Some(application_quit_mainloop::<T>);
            klass.run_mainloop = Some(application_run_mainloop::<T>);
            klass.shutdown = Some(application_shutdown::<T>);
            klass.startup = Some(application_startup::<T>);
        }
    }
}


unsafe extern "C" fn application_activate<T: ObjectSubclass>
(ptr: *mut ffi::GApplication)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.activate(&wrap);
}

unsafe extern "C" fn application_after_emit<T: ObjectSubclass>
(ptr: *mut ffi::GApplication, platform_data: *mut glib_ffi::GVariant)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.after_emit(&wrap, &from_glib_none(platform_data));
}

unsafe extern "C" fn application_before_emit<T: ObjectSubclass>
(ptr: *mut ffi::GApplication, platform_data: *mut glib_ffi::GVariant)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.before_emit(&wrap, &from_glib_none(platform_data));
}

unsafe extern "C" fn application_command_line<T: ObjectSubclass>
(ptr: *mut ffi::GApplication, command_line: *mut ffi::GApplicationCommandLine) -> libc::c_int
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    let rs_ret = imp.command_line(&wrap, &from_glib_none(command_line));
    rs_ret
}

unsafe extern "C" fn application_local_command_line<T: ObjectSubclass>(
    ptr: *mut ffi::GApplication,
    arguments: *mut *mut *mut libc::c_char,
    exit_status: *mut libc::c_int,
) -> glib_ffi::gboolean
where
    T: ApplicationImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    let mut args = ArgumentList::new(arguments);
    match imp.local_command_line(&wrap, &mut args) {
        Some(ret) => {
            ptr::write(exit_status, ret);
            glib_ffi::GTRUE
        }
        None => glib_ffi::GFALSE,
    }
}

unsafe extern "C" fn application_open<T: ObjectSubclass>(
    ptr: *mut ffi::GApplication,
    files: *mut *mut ffi::GFile,
    num_files: libc::c_int,
    hint: *const libc::c_char,
) where
    T: ApplicationImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    let files_r: Vec<File> = FromGlibContainer::from_glib_none_num(files, num_files as usize);
    let hint_r: String = from_glib_none(hint);
    imp.open(&wrap, &files_r.as_slice(), &hint_r.as_str())
}

unsafe extern "C" fn application_quit_mainloop<T: ObjectSubclass>
(ptr: *mut ffi::GApplication)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.quit_mainloop(&wrap);
}

unsafe extern "C" fn application_run_mainloop<T: ObjectSubclass>
(ptr: *mut ffi::GApplication)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.run_mainloop(&wrap);
}

unsafe extern "C" fn application_shutdown<T: ObjectSubclass>
(ptr: *mut ffi::GApplication)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.shutdown(&wrap);
}

unsafe extern "C" fn application_startup<T: ObjectSubclass>
(ptr: *mut ffi::GApplication)
where
    T: ApplicationImpl
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);
    imp.startup(&wrap);
}



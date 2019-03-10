// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use FileInfo;
use OutputStream;
use Seekable;
#[cfg(feature = "futures")]
use futures_core;
use gio_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct FileOutputStream(Object<gio_sys::GFileOutputStream, gio_sys::GFileOutputStreamClass, FileOutputStreamClass>) @extends OutputStream, @implements Seekable;

    match fn {
        get_type => || gio_sys::g_file_output_stream_get_type(),
    }
}

pub const NONE_FILE_OUTPUT_STREAM: Option<&FileOutputStream> = None;

pub trait FileOutputStreamExt: 'static {
    fn get_etag(&self) -> Option<GString>;

    fn query_info<P: IsA<Cancellable>>(&self, attributes: &str, cancellable: Option<&P>) -> Result<FileInfo, Error>;

    fn query_info_async<P: IsA<Cancellable>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn query_info_async_future(&self, attributes: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, FileInfo), Error = (Self, Error)>> where Self: Sized + Clone;
}

impl<O: IsA<FileOutputStream>> FileOutputStreamExt for O {
    fn get_etag(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_file_output_stream_get_etag(self.as_ref().to_glib_none().0))
        }
    }

    fn query_info<P: IsA<Cancellable>>(&self, attributes: &str, cancellable: Option<&P>) -> Result<FileInfo, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_file_output_stream_query_info(self.as_ref().to_glib_none().0, attributes.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn query_info_async<P: IsA<Cancellable>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn query_info_async_trampoline<Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_file_output_stream_query_info_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_info_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_file_output_stream_query_info_async(self.as_ref().to_glib_none().0, attributes.to_glib_none().0, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn query_info_async_future(&self, attributes: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, FileInfo), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let attributes = String::from(attributes);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.query_info_async(
                &attributes,
                io_priority,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

impl fmt::Display for FileOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileOutputStream")
    }
}

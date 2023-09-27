// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::OutputStream;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GFilterOutputStream")]
    pub struct FilterOutputStream(Object<ffi::GFilterOutputStream, ffi::GFilterOutputStreamClass>) @extends OutputStream;

    match fn {
        type_ => || ffi::g_filter_output_stream_get_type(),
    }
}

impl FilterOutputStream {
    pub const NONE: Option<&'static FilterOutputStream> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::FilterOutputStream>> Sealed for T {}
}

pub trait FilterOutputStreamExt: IsA<FilterOutputStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_filter_output_stream_get_base_stream")]
    #[doc(alias = "get_base_stream")]
    fn base_stream(&self) -> OutputStream {
        unsafe {
            from_glib_none(ffi::g_filter_output_stream_get_base_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_filter_output_stream_get_close_base_stream")]
    #[doc(alias = "get_close_base_stream")]
    fn closes_base_stream(&self) -> bool {
        unsafe {
            from_glib(ffi::g_filter_output_stream_get_close_base_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_filter_output_stream_set_close_base_stream")]
    fn set_close_base_stream(&self, close_base: bool) {
        unsafe {
            ffi::g_filter_output_stream_set_close_base_stream(
                self.as_ref().to_glib_none().0,
                close_base.into_glib(),
            );
        }
    }

    #[doc(alias = "close-base-stream")]
    fn connect_close_base_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_close_base_stream_trampoline<
            P: IsA<FilterOutputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GFilterOutputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FilterOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::close-base-stream\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_close_base_stream_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<FilterOutputStream>> FilterOutputStreamExt for O {}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Cancellable, OutputStream};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GPollableOutputStream")]
    pub struct PollableOutputStream(Interface<ffi::GPollableOutputStream, ffi::GPollableOutputStreamInterface>) @requires OutputStream;

    match fn {
        type_ => || ffi::g_pollable_output_stream_get_type(),
    }
}

impl PollableOutputStream {
    pub const NONE: Option<&'static PollableOutputStream> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PollableOutputStream>> Sealed for T {}
}

pub trait PollableOutputStreamExt: IsA<PollableOutputStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_pollable_output_stream_can_poll")]
    fn can_poll(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_output_stream_can_poll(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_pollable_output_stream_is_writable")]
    fn is_writable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_output_stream_is_writable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_pollable_output_stream_write_nonblocking")]
    fn write_nonblocking(
        &self,
        buffer: &[u8],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        let count = buffer.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_pollable_output_stream_write_nonblocking(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<PollableOutputStream>> PollableOutputStreamExt for O {}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{File, InputStream};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GApplicationCommandLine")]
    pub struct ApplicationCommandLine(Object<ffi::GApplicationCommandLine, ffi::GApplicationCommandLineClass>);

    match fn {
        type_ => || ffi::g_application_command_line_get_type(),
    }
}

impl ApplicationCommandLine {
    pub const NONE: Option<&'static ApplicationCommandLine> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ApplicationCommandLine>> Sealed for T {}
}

pub trait ApplicationCommandLineExt:
    IsA<ApplicationCommandLine> + sealed::Sealed + 'static
{
    #[doc(alias = "g_application_command_line_create_file_for_arg")]
    fn create_file_for_arg(&self, arg: impl AsRef<std::ffi::OsStr>) -> File {
        unsafe {
            from_glib_full(ffi::g_application_command_line_create_file_for_arg(
                self.as_ref().to_glib_none().0,
                arg.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_get_arguments")]
    #[doc(alias = "get_arguments")]
    fn arguments(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            let mut argc = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::g_application_command_line_get_arguments(
                    self.as_ref().to_glib_none().0,
                    argc.as_mut_ptr(),
                ),
                argc.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "g_application_command_line_get_cwd")]
    #[doc(alias = "get_cwd")]
    fn cwd(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_application_command_line_get_cwd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_get_environ")]
    #[doc(alias = "get_environ")]
    fn environ(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_application_command_line_get_environ(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_get_exit_status")]
    #[doc(alias = "get_exit_status")]
    fn exit_status(&self) -> i32 {
        unsafe { ffi::g_application_command_line_get_exit_status(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_application_command_line_get_is_remote")]
    #[doc(alias = "get_is_remote")]
    fn is_remote(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_command_line_get_is_remote(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_get_options_dict")]
    #[doc(alias = "get_options_dict")]
    fn options_dict(&self) -> glib::VariantDict {
        unsafe {
            from_glib_none(ffi::g_application_command_line_get_options_dict(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_get_platform_data")]
    #[doc(alias = "get_platform_data")]
    fn platform_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_application_command_line_get_platform_data(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_get_stdin")]
    #[doc(alias = "get_stdin")]
    fn stdin(&self) -> Option<InputStream> {
        unsafe {
            from_glib_full(ffi::g_application_command_line_get_stdin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_application_command_line_getenv")]
    fn getenv(&self, name: impl AsRef<std::ffi::OsStr>) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_application_command_line_getenv(
                self.as_ref().to_glib_none().0,
                name.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_application_command_line_print")]
    //fn print(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:g_application_command_line_print() }
    //}

    //#[doc(alias = "g_application_command_line_printerr")]
    //fn printerr(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:g_application_command_line_printerr() }
    //}

    #[doc(alias = "g_application_command_line_set_exit_status")]
    fn set_exit_status(&self, exit_status: i32) {
        unsafe {
            ffi::g_application_command_line_set_exit_status(
                self.as_ref().to_glib_none().0,
                exit_status,
            );
        }
    }

    #[doc(alias = "is-remote")]
    fn connect_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_remote_trampoline<
            P: IsA<ApplicationCommandLine>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GApplicationCommandLine,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ApplicationCommandLine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-remote\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_is_remote_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ApplicationCommandLine>> ApplicationCommandLineExt for O {}

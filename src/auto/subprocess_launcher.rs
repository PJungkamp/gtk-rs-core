// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_40", feature = "dox"))]
use Error;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Subprocess;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessFlags;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SubprocessLauncher(Object<ffi::GSubprocessLauncher>);

    match fn {
        get_type => || ffi::g_subprocess_launcher_get_type(),
    }
}

impl SubprocessLauncher {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn new(flags: SubprocessFlags) -> SubprocessLauncher {
        unsafe {
            from_glib_full(ffi::g_subprocess_launcher_new(flags.to_glib()))
        }
    }
}

pub trait SubprocessLauncherExt {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn getenv<P: AsRef<std::path::Path>>(&self, variable: P) -> Option<std::path::PathBuf>;

    //#[cfg(any(unix, feature = "dox"))]
    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn set_child_setup(&self, child_setup: /*Unknown conversion*//*Unimplemented*/SpawnChildSetupFunc, destroy_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_cwd<P: AsRef<std::path::Path>>(&self, cwd: P);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_environ(&self, env: &[&std::path::Path]);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_flags(&self, flags: SubprocessFlags);

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stderr_file_path<P: AsRef<std::path::Path>>(&self, path: P);

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdin_file_path(&self, path: &str);

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdout_file_path<P: AsRef<std::path::Path>>(&self, path: P);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn setenv<P: AsRef<std::ffi::OsStr>, Q: AsRef<std::ffi::OsStr>>(&self, variable: P, value: Q, overwrite: bool);

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn spawn(&self, error: &mut Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Subprocess>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn spawnv(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, Error>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn unsetenv<P: AsRef<std::ffi::OsStr>>(&self, variable: P);

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SubprocessLauncher> + IsA<glib::object::Object>> SubprocessLauncherExt for O {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn getenv<P: AsRef<std::path::Path>>(&self, variable: P) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_subprocess_launcher_getenv(self.to_glib_none().0, variable.as_ref().to_glib_none().0))
        }
    }

    //#[cfg(any(unix, feature = "dox"))]
    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn set_child_setup(&self, child_setup: /*Unknown conversion*//*Unimplemented*/SpawnChildSetupFunc, destroy_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::g_subprocess_launcher_set_child_setup() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_cwd<P: AsRef<std::path::Path>>(&self, cwd: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_cwd(self.to_glib_none().0, cwd.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_environ(&self, env: &[&std::path::Path]) {
        unsafe {
            ffi::g_subprocess_launcher_set_environ(self.to_glib_none().0, env.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_flags(&self, flags: SubprocessFlags) {
        unsafe {
            ffi::g_subprocess_launcher_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stderr_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stderr_file_path(self.to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdin_file_path(&self, path: &str) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdin_file_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdout_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdout_file_path(self.to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn setenv<P: AsRef<std::ffi::OsStr>, Q: AsRef<std::ffi::OsStr>>(&self, variable: P, value: Q, overwrite: bool) {
        unsafe {
            ffi::g_subprocess_launcher_setenv(self.to_glib_none().0, variable.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0, overwrite.to_glib());
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn spawn(&self, error: &mut Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Subprocess> {
    //    unsafe { TODO: call ffi::g_subprocess_launcher_spawn() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn spawnv(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_launcher_spawnv(self.to_glib_none().0, argv.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn unsetenv<P: AsRef<std::ffi::OsStr>>(&self, variable: P) {
        unsafe {
            ffi::g_subprocess_launcher_unsetenv(self.to_glib_none().0, variable.as_ref().to_glib_none().0);
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::GSubprocessLauncher, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SubprocessLauncher> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SubprocessLauncher::from_glib_borrow(this).downcast_unchecked())
}

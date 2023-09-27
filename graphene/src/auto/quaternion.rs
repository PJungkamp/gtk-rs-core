// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Matrix, Vec3, Vec4};
use glib::translate::*;

glib::wrapper! {
    pub struct Quaternion(BoxedInline<ffi::graphene_quaternion_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_quaternion_get_type(), ptr as *mut _) as *mut ffi::graphene_quaternion_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_quaternion_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_quaternion_get_type(),
    }
}

impl Quaternion {
    #[doc(alias = "graphene_quaternion_add")]
    #[must_use]
    pub fn add(&self, b: &Quaternion) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_quaternion_add(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_quaternion_dot")]
    pub fn dot(&self, b: &Quaternion) -> f32 {
        unsafe { ffi::graphene_quaternion_dot(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_quaternion_equal")]
    fn equal(&self, b: &Quaternion) -> bool {
        unsafe { ffi::graphene_quaternion_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_quaternion_invert")]
    #[must_use]
    pub fn invert(&self) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_quaternion_invert(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_quaternion_multiply")]
    #[must_use]
    pub fn multiply(&self, b: &Quaternion) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_quaternion_multiply(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_quaternion_normalize")]
    #[must_use]
    pub fn normalize(&self) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_quaternion_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_quaternion_scale")]
    #[must_use]
    pub fn scale(&self, factor: f32) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_quaternion_scale(self.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_quaternion_slerp")]
    #[must_use]
    pub fn slerp(&self, b: &Quaternion, factor: f32) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_quaternion_slerp(
                self.to_glib_none().0,
                b.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_quaternion_to_angle_vec3")]
    pub fn to_angle_vec3(&self) -> (f32, Vec3) {
        unsafe {
            let mut angle = std::mem::MaybeUninit::uninit();
            let mut axis = Vec3::uninitialized();
            ffi::graphene_quaternion_to_angle_vec3(
                self.to_glib_none().0,
                angle.as_mut_ptr(),
                axis.to_glib_none_mut().0,
            );
            (angle.assume_init(), axis)
        }
    }

    #[doc(alias = "graphene_quaternion_to_angles")]
    pub fn to_angles(&self) -> (f32, f32, f32) {
        unsafe {
            let mut deg_x = std::mem::MaybeUninit::uninit();
            let mut deg_y = std::mem::MaybeUninit::uninit();
            let mut deg_z = std::mem::MaybeUninit::uninit();
            ffi::graphene_quaternion_to_angles(
                self.to_glib_none().0,
                deg_x.as_mut_ptr(),
                deg_y.as_mut_ptr(),
                deg_z.as_mut_ptr(),
            );
            (
                deg_x.assume_init(),
                deg_y.assume_init(),
                deg_z.assume_init(),
            )
        }
    }

    #[doc(alias = "graphene_quaternion_to_matrix")]
    pub fn to_matrix(&self) -> Matrix {
        unsafe {
            let mut m = Matrix::uninitialized();
            ffi::graphene_quaternion_to_matrix(self.to_glib_none().0, m.to_glib_none_mut().0);
            m
        }
    }

    #[doc(alias = "graphene_quaternion_to_radians")]
    pub fn to_radians(&self) -> (f32, f32, f32) {
        unsafe {
            let mut rad_x = std::mem::MaybeUninit::uninit();
            let mut rad_y = std::mem::MaybeUninit::uninit();
            let mut rad_z = std::mem::MaybeUninit::uninit();
            ffi::graphene_quaternion_to_radians(
                self.to_glib_none().0,
                rad_x.as_mut_ptr(),
                rad_y.as_mut_ptr(),
                rad_z.as_mut_ptr(),
            );
            (
                rad_x.assume_init(),
                rad_y.assume_init(),
                rad_z.assume_init(),
            )
        }
    }

    #[doc(alias = "graphene_quaternion_to_vec4")]
    pub fn to_vec4(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_quaternion_to_vec4(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }
}

impl PartialEq for Quaternion {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Quaternion {}

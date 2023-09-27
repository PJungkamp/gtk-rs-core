// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Color, Font, Glyph, GlyphItem, GlyphString, Layout, LayoutLine, Matrix, RenderPart};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "PangoRenderer")]
    pub struct Renderer(Object<ffi::PangoRenderer, ffi::PangoRendererClass>);

    match fn {
        type_ => || ffi::pango_renderer_get_type(),
    }
}

impl Renderer {
    pub const NONE: Option<&'static Renderer> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Renderer>> Sealed for T {}
}

pub trait RendererExt: IsA<Renderer> + sealed::Sealed + 'static {
    #[doc(alias = "pango_renderer_activate")]
    fn activate(&self) {
        unsafe {
            ffi::pango_renderer_activate(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "pango_renderer_deactivate")]
    fn deactivate(&self) {
        unsafe {
            ffi::pango_renderer_deactivate(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "pango_renderer_draw_error_underline")]
    fn draw_error_underline(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::pango_renderer_draw_error_underline(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_glyph")]
    fn draw_glyph(&self, font: &impl IsA<Font>, glyph: Glyph, x: f64, y: f64) {
        unsafe {
            ffi::pango_renderer_draw_glyph(
                self.as_ref().to_glib_none().0,
                font.as_ref().to_glib_none().0,
                glyph,
                x,
                y,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_glyph_item")]
    fn draw_glyph_item(&self, text: Option<&str>, glyph_item: &mut GlyphItem, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_glyph_item(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                glyph_item.to_glib_none_mut().0,
                x,
                y,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_glyphs")]
    fn draw_glyphs(&self, font: &impl IsA<Font>, glyphs: &mut GlyphString, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_glyphs(
                self.as_ref().to_glib_none().0,
                font.as_ref().to_glib_none().0,
                glyphs.to_glib_none_mut().0,
                x,
                y,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_layout")]
    fn draw_layout(&self, layout: &Layout, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_layout(
                self.as_ref().to_glib_none().0,
                layout.to_glib_none().0,
                x,
                y,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_layout_line")]
    fn draw_layout_line(&self, line: &LayoutLine, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_layout_line(
                self.as_ref().to_glib_none().0,
                line.to_glib_none().0,
                x,
                y,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_rectangle")]
    fn draw_rectangle(&self, part: RenderPart, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::pango_renderer_draw_rectangle(
                self.as_ref().to_glib_none().0,
                part.into_glib(),
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "pango_renderer_draw_trapezoid")]
    fn draw_trapezoid(
        &self,
        part: RenderPart,
        y1_: f64,
        x11: f64,
        x21: f64,
        y2: f64,
        x12: f64,
        x22: f64,
    ) {
        unsafe {
            ffi::pango_renderer_draw_trapezoid(
                self.as_ref().to_glib_none().0,
                part.into_glib(),
                y1_,
                x11,
                x21,
                y2,
                x12,
                x22,
            );
        }
    }

    #[doc(alias = "pango_renderer_get_alpha")]
    #[doc(alias = "get_alpha")]
    fn alpha(&self, part: RenderPart) -> u16 {
        unsafe { ffi::pango_renderer_get_alpha(self.as_ref().to_glib_none().0, part.into_glib()) }
    }

    #[doc(alias = "pango_renderer_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self, part: RenderPart) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_color(
                self.as_ref().to_glib_none().0,
                part.into_glib(),
            ))
        }
    }

    #[doc(alias = "pango_renderer_get_layout")]
    #[doc(alias = "get_layout")]
    fn layout(&self) -> Option<Layout> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_layout(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_renderer_get_layout_line")]
    #[doc(alias = "get_layout_line")]
    fn layout_line(&self) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_layout_line(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_renderer_get_matrix")]
    #[doc(alias = "get_matrix")]
    fn matrix(&self) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_matrix(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_renderer_part_changed")]
    fn part_changed(&self, part: RenderPart) {
        unsafe {
            ffi::pango_renderer_part_changed(self.as_ref().to_glib_none().0, part.into_glib());
        }
    }

    #[doc(alias = "pango_renderer_set_alpha")]
    fn set_alpha(&self, part: RenderPart, alpha: u16) {
        unsafe {
            ffi::pango_renderer_set_alpha(self.as_ref().to_glib_none().0, part.into_glib(), alpha);
        }
    }

    #[doc(alias = "pango_renderer_set_color")]
    fn set_color(&self, part: RenderPart, color: Option<&Color>) {
        unsafe {
            ffi::pango_renderer_set_color(
                self.as_ref().to_glib_none().0,
                part.into_glib(),
                color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "pango_renderer_set_matrix")]
    fn set_matrix(&self, matrix: Option<&Matrix>) {
        unsafe {
            ffi::pango_renderer_set_matrix(self.as_ref().to_glib_none().0, matrix.to_glib_none().0);
        }
    }
}

impl<O: IsA<Renderer>> RendererExt for O {}

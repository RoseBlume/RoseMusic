// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v1_46")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
use crate::Direction;
#[cfg(feature = "v1_50")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
use crate::LayoutSerializeFlags;
use crate::{
    Alignment, AttrList, Context, EllipsizeMode, FontDescription, LayoutIter, LayoutLine,
    Rectangle, TabArray, WrapMode,
};
use glib::translate::*;
#[cfg(feature = "v1_50")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
use std::ptr;
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "PangoLayout")]
    pub struct Layout(Object<ffi::PangoLayout, ffi::PangoLayoutClass>);

    match fn {
        type_ => || ffi::pango_layout_get_type(),
    }
}

impl Layout {
    #[doc(alias = "pango_layout_new")]
    pub fn new(context: &Context) -> Layout {
        unsafe { from_glib_full(ffi::pango_layout_new(context.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_context_changed")]
    pub fn context_changed(&self) {
        unsafe {
            ffi::pango_layout_context_changed(self.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_layout_copy")]
    #[must_use]
    pub fn copy(&self) -> Layout {
        unsafe { from_glib_full(ffi::pango_layout_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_alignment")]
    #[doc(alias = "get_alignment")]
    pub fn alignment(&self) -> Alignment {
        unsafe { from_glib(ffi::pango_layout_get_alignment(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_attributes")]
    #[doc(alias = "get_attributes")]
    pub fn attributes(&self) -> Option<AttrList> {
        unsafe { from_glib_none(ffi::pango_layout_get_attributes(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_auto_dir")]
    #[doc(alias = "get_auto_dir")]
    pub fn is_auto_dir(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_get_auto_dir(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_baseline")]
    #[doc(alias = "get_baseline")]
    pub fn baseline(&self) -> i32 {
        unsafe { ffi::pango_layout_get_baseline(self.to_glib_none().0) }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_get_caret_pos")]
    #[doc(alias = "get_caret_pos")]
    pub fn caret_pos(&self, index_: i32) -> (Rectangle, Rectangle) {
        unsafe {
            let mut strong_pos = Rectangle::uninitialized();
            let mut weak_pos = Rectangle::uninitialized();
            ffi::pango_layout_get_caret_pos(
                self.to_glib_none().0,
                index_,
                strong_pos.to_glib_none_mut().0,
                weak_pos.to_glib_none_mut().0,
            );
            (strong_pos, weak_pos)
        }
    }

    #[doc(alias = "pango_layout_get_character_count")]
    #[doc(alias = "get_character_count")]
    pub fn character_count(&self) -> i32 {
        unsafe { ffi::pango_layout_get_character_count(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> Context {
        unsafe { from_glib_none(ffi::pango_layout_get_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_cursor_pos")]
    #[doc(alias = "get_cursor_pos")]
    pub fn cursor_pos(&self, index_: i32) -> (Rectangle, Rectangle) {
        unsafe {
            let mut strong_pos = Rectangle::uninitialized();
            let mut weak_pos = Rectangle::uninitialized();
            ffi::pango_layout_get_cursor_pos(
                self.to_glib_none().0,
                index_,
                strong_pos.to_glib_none_mut().0,
                weak_pos.to_glib_none_mut().0,
            );
            (strong_pos, weak_pos)
        }
    }

    #[cfg(feature = "v1_46")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_layout_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self, index: i32) -> Direction {
        unsafe {
            from_glib(ffi::pango_layout_get_direction(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_ellipsize")]
    #[doc(alias = "get_ellipsize")]
    pub fn ellipsize(&self) -> EllipsizeMode {
        unsafe { from_glib(ffi::pango_layout_get_ellipsize(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_extents")]
    #[doc(alias = "get_extents")]
    pub fn extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_get_font_description")]
    #[doc(alias = "get_font_description")]
    pub fn font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_font_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::pango_layout_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_indent")]
    #[doc(alias = "get_indent")]
    pub fn indent(&self) -> i32 {
        unsafe { ffi::pango_layout_get_indent(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_iter")]
    #[doc(alias = "get_iter")]
    pub fn iter(&self) -> LayoutIter {
        unsafe { from_glib_full(ffi::pango_layout_get_iter(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_justify")]
    #[doc(alias = "get_justify")]
    pub fn is_justify(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_get_justify(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_get_justify_last_line")]
    #[doc(alias = "get_justify_last_line")]
    pub fn is_justify_last_line(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_justify_last_line(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_line")]
    #[doc(alias = "get_line")]
    pub fn line(&self, line: i32) -> Option<LayoutLine> {
        unsafe { from_glib_none(ffi::pango_layout_get_line(self.to_glib_none().0, line)) }
    }

    #[doc(alias = "pango_layout_get_line_count")]
    #[doc(alias = "get_line_count")]
    pub fn line_count(&self) -> i32 {
        unsafe { ffi::pango_layout_get_line_count(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_line_readonly")]
    #[doc(alias = "get_line_readonly")]
    pub fn line_readonly(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_line_readonly(
                self.to_glib_none().0,
                line,
            ))
        }
    }

    #[cfg(feature = "v1_44")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_layout_get_line_spacing")]
    #[doc(alias = "get_line_spacing")]
    pub fn line_spacing(&self) -> f32 {
        unsafe { ffi::pango_layout_get_line_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_lines")]
    #[doc(alias = "get_lines")]
    pub fn lines(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines(self.to_glib_none().0))
        }
    }

    #[doc(alias = "pango_layout_get_lines_readonly")]
    #[doc(alias = "get_lines_readonly")]
    pub fn lines_readonly(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines_readonly(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "pango_layout_get_log_attrs")]
    //#[doc(alias = "get_log_attrs")]
    //pub fn log_attrs(&self, attrs: /*Ignored*/Vec<LogAttr>) -> i32 {
    //    unsafe { TODO: call ffi:pango_layout_get_log_attrs() }
    //}

    //#[doc(alias = "pango_layout_get_log_attrs_readonly")]
    //#[doc(alias = "get_log_attrs_readonly")]
    //pub fn log_attrs_readonly(&self) -> /*Ignored*/Vec<LogAttr> {
    //    unsafe { TODO: call ffi:pango_layout_get_log_attrs_readonly() }
    //}

    #[doc(alias = "pango_layout_get_pixel_extents")]
    #[doc(alias = "get_pixel_extents")]
    pub fn pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_pixel_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_get_pixel_size")]
    #[doc(alias = "get_pixel_size")]
    pub fn pixel_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::pango_layout_get_pixel_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "pango_layout_get_serial")]
    #[doc(alias = "get_serial")]
    pub fn serial(&self) -> u32 {
        unsafe { ffi::pango_layout_get_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_single_paragraph_mode")]
    #[doc(alias = "get_single_paragraph_mode")]
    pub fn is_single_paragraph_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_single_paragraph_mode(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::pango_layout_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "pango_layout_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> i32 {
        unsafe { ffi::pango_layout_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_tabs")]
    #[doc(alias = "get_tabs")]
    pub fn tabs(&self) -> Option<TabArray> {
        unsafe { from_glib_full(ffi::pango_layout_get_tabs(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_text")]
    #[doc(alias = "get_text")]
    pub fn text(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::pango_layout_get_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_unknown_glyphs_count")]
    #[doc(alias = "get_unknown_glyphs_count")]
    pub fn unknown_glyphs_count(&self) -> i32 {
        unsafe { ffi::pango_layout_get_unknown_glyphs_count(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::pango_layout_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_wrap")]
    #[doc(alias = "get_wrap")]
    pub fn wrap(&self) -> WrapMode {
        unsafe { from_glib(ffi::pango_layout_get_wrap(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_index_to_line_x")]
    pub fn index_to_line_x(&self, index_: i32, trailing: bool) -> (i32, i32) {
        unsafe {
            let mut line = mem::MaybeUninit::uninit();
            let mut x_pos = mem::MaybeUninit::uninit();
            ffi::pango_layout_index_to_line_x(
                self.to_glib_none().0,
                index_,
                trailing.into_glib(),
                line.as_mut_ptr(),
                x_pos.as_mut_ptr(),
            );
            (line.assume_init(), x_pos.assume_init())
        }
    }

    #[doc(alias = "pango_layout_index_to_pos")]
    pub fn index_to_pos(&self, index_: i32) -> Rectangle {
        unsafe {
            let mut pos = Rectangle::uninitialized();
            ffi::pango_layout_index_to_pos(self.to_glib_none().0, index_, pos.to_glib_none_mut().0);
            pos
        }
    }

    #[doc(alias = "pango_layout_is_ellipsized")]
    pub fn is_ellipsized(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_is_ellipsized(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_is_wrapped")]
    pub fn is_wrapped(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_is_wrapped(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_move_cursor_visually")]
    pub fn move_cursor_visually(
        &self,
        strong: bool,
        old_index: i32,
        old_trailing: i32,
        direction: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut new_index = mem::MaybeUninit::uninit();
            let mut new_trailing = mem::MaybeUninit::uninit();
            ffi::pango_layout_move_cursor_visually(
                self.to_glib_none().0,
                strong.into_glib(),
                old_index,
                old_trailing,
                direction,
                new_index.as_mut_ptr(),
                new_trailing.as_mut_ptr(),
            );
            (new_index.assume_init(), new_trailing.assume_init())
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_serialize")]
    pub fn serialize(&self, flags: LayoutSerializeFlags) -> glib::Bytes {
        unsafe {
            from_glib_full(ffi::pango_layout_serialize(
                self.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "pango_layout_set_alignment")]
    pub fn set_alignment(&self, alignment: Alignment) {
        unsafe {
            ffi::pango_layout_set_alignment(self.to_glib_none().0, alignment.into_glib());
        }
    }

    #[doc(alias = "pango_layout_set_attributes")]
    pub fn set_attributes(&self, attrs: Option<&AttrList>) {
        unsafe {
            ffi::pango_layout_set_attributes(self.to_glib_none().0, attrs.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_layout_set_auto_dir")]
    pub fn set_auto_dir(&self, auto_dir: bool) {
        unsafe {
            ffi::pango_layout_set_auto_dir(self.to_glib_none().0, auto_dir.into_glib());
        }
    }

    #[doc(alias = "pango_layout_set_ellipsize")]
    pub fn set_ellipsize(&self, ellipsize: EllipsizeMode) {
        unsafe {
            ffi::pango_layout_set_ellipsize(self.to_glib_none().0, ellipsize.into_glib());
        }
    }

    #[doc(alias = "pango_layout_set_font_description")]
    pub fn set_font_description(&self, desc: Option<&FontDescription>) {
        unsafe {
            ffi::pango_layout_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_layout_set_height")]
    pub fn set_height(&self, height: i32) {
        unsafe {
            ffi::pango_layout_set_height(self.to_glib_none().0, height);
        }
    }

    #[doc(alias = "pango_layout_set_indent")]
    pub fn set_indent(&self, indent: i32) {
        unsafe {
            ffi::pango_layout_set_indent(self.to_glib_none().0, indent);
        }
    }

    #[doc(alias = "pango_layout_set_justify")]
    pub fn set_justify(&self, justify: bool) {
        unsafe {
            ffi::pango_layout_set_justify(self.to_glib_none().0, justify.into_glib());
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_set_justify_last_line")]
    pub fn set_justify_last_line(&self, justify: bool) {
        unsafe {
            ffi::pango_layout_set_justify_last_line(self.to_glib_none().0, justify.into_glib());
        }
    }

    #[cfg(feature = "v1_44")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_layout_set_line_spacing")]
    pub fn set_line_spacing(&self, factor: f32) {
        unsafe {
            ffi::pango_layout_set_line_spacing(self.to_glib_none().0, factor);
        }
    }

    #[doc(alias = "pango_layout_set_markup")]
    pub fn set_markup(&self, markup: &str) {
        let length = markup.len() as _;
        unsafe {
            ffi::pango_layout_set_markup(self.to_glib_none().0, markup.to_glib_none().0, length);
        }
    }

    #[doc(alias = "pango_layout_set_markup_with_accel")]
    pub fn set_markup_with_accel(&self, markup: &str, accel_marker: char) -> char {
        let length = markup.len() as _;
        unsafe {
            let mut accel_char = mem::MaybeUninit::uninit();
            ffi::pango_layout_set_markup_with_accel(
                self.to_glib_none().0,
                markup.to_glib_none().0,
                length,
                accel_marker.into_glib(),
                accel_char.as_mut_ptr(),
            );
            std::convert::TryFrom::try_from(accel_char.assume_init())
                .expect("conversion from an invalid Unicode value attempted")
        }
    }

    #[doc(alias = "pango_layout_set_single_paragraph_mode")]
    pub fn set_single_paragraph_mode(&self, setting: bool) {
        unsafe {
            ffi::pango_layout_set_single_paragraph_mode(self.to_glib_none().0, setting.into_glib());
        }
    }

    #[doc(alias = "pango_layout_set_spacing")]
    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::pango_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "pango_layout_set_tabs")]
    pub fn set_tabs(&self, tabs: Option<&TabArray>) {
        unsafe {
            ffi::pango_layout_set_tabs(self.to_glib_none().0, mut_override(tabs.to_glib_none().0));
        }
    }

    #[doc(alias = "pango_layout_set_text")]
    pub fn set_text(&self, text: &str) {
        let length = text.len() as _;
        unsafe {
            ffi::pango_layout_set_text(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    #[doc(alias = "pango_layout_set_width")]
    pub fn set_width(&self, width: i32) {
        unsafe {
            ffi::pango_layout_set_width(self.to_glib_none().0, width);
        }
    }

    #[doc(alias = "pango_layout_set_wrap")]
    pub fn set_wrap(&self, wrap: WrapMode) {
        unsafe {
            ffi::pango_layout_set_wrap(self.to_glib_none().0, wrap.into_glib());
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_write_to_file")]
    pub fn write_to_file(
        &self,
        flags: LayoutSerializeFlags,
        filename: impl AsRef<std::path::Path>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::pango_layout_write_to_file(
                self.to_glib_none().0,
                flags.into_glib(),
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "pango_layout_xy_to_index")]
    pub fn xy_to_index(&self, x: i32, y: i32) -> (bool, i32, i32) {
        unsafe {
            let mut index_ = mem::MaybeUninit::uninit();
            let mut trailing = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::pango_layout_xy_to_index(
                self.to_glib_none().0,
                x,
                y,
                index_.as_mut_ptr(),
                trailing.as_mut_ptr(),
            ));
            (ret, index_.assume_init(), trailing.assume_init())
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Layout")
    }
}
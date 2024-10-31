// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v1_50")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
use crate::Direction;
use crate::Rectangle;
use glib::translate::*;
use std::{mem, ptr};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayoutLine(Shared<ffi::PangoLayoutLine>);

    match fn {
        ref => |ptr| ffi::pango_layout_line_ref(ptr),
        unref => |ptr| ffi::pango_layout_line_unref(ptr),
        type_ => || ffi::pango_layout_line_get_type(),
    }
}

impl LayoutLine {
    #[doc(alias = "pango_layout_line_get_extents")]
    #[doc(alias = "get_extents")]
    pub fn extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_line_get_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[cfg(feature = "v1_44")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_layout_line_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe {
            let mut height = mem::MaybeUninit::uninit();
            ffi::pango_layout_line_get_height(self.to_glib_none().0, height.as_mut_ptr());
            height.assume_init()
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_line_get_length")]
    #[doc(alias = "get_length")]
    pub fn length(&self) -> i32 {
        unsafe { ffi::pango_layout_line_get_length(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_line_get_pixel_extents")]
    #[doc(alias = "get_pixel_extents")]
    pub fn pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_line_get_pixel_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_line_get_resolved_direction")]
    #[doc(alias = "get_resolved_direction")]
    pub fn resolved_direction(&self) -> Direction {
        unsafe {
            from_glib(ffi::pango_layout_line_get_resolved_direction(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_line_get_start_index")]
    #[doc(alias = "get_start_index")]
    pub fn start_index(&self) -> i32 {
        unsafe { ffi::pango_layout_line_get_start_index(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_line_get_x_ranges")]
    #[doc(alias = "get_x_ranges")]
    pub fn x_ranges(&self, start_index: i32, end_index: i32) -> Vec<i32> {
        unsafe {
            let mut ranges = ptr::null_mut();
            let mut n_ranges = mem::MaybeUninit::uninit();
            ffi::pango_layout_line_get_x_ranges(
                self.to_glib_none().0,
                start_index,
                end_index,
                &mut ranges,
                n_ranges.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_full_num(ranges, n_ranges.assume_init() as _)
        }
    }

    #[doc(alias = "pango_layout_line_index_to_x")]
    pub fn index_to_x(&self, index_: i32, trailing: bool) -> i32 {
        unsafe {
            let mut x_pos = mem::MaybeUninit::uninit();
            ffi::pango_layout_line_index_to_x(
                self.to_glib_none().0,
                index_,
                trailing.into_glib(),
                x_pos.as_mut_ptr(),
            );
            x_pos.assume_init()
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_line_is_paragraph_start")]
    pub fn is_paragraph_start(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_line_is_paragraph_start(
                self.to_glib_none().0,
            ))
        }
    }
}

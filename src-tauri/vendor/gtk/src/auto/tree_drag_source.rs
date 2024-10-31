// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{SelectionData, TreePath};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkTreeDragSource")]
    pub struct TreeDragSource(Interface<ffi::GtkTreeDragSource, ffi::GtkTreeDragSourceIface>);

    match fn {
        type_ => || ffi::gtk_tree_drag_source_get_type(),
    }
}

impl TreeDragSource {
    pub const NONE: Option<&'static TreeDragSource> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TreeDragSource>> Sealed for T {}
}

pub trait TreeDragSourceExt: IsA<TreeDragSource> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_tree_drag_source_drag_data_delete")]
    fn drag_data_delete(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_drag_data_delete(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_drag_source_drag_data_get")]
    fn drag_data_get(&self, path: &mut TreePath, selection_data: &mut SelectionData) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_drag_data_get(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
                selection_data.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_drag_source_row_draggable")]
    fn row_draggable(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_row_draggable(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            ))
        }
    }
}

impl<O: IsA<TreeDragSource>> TreeDragSourceExt for O {}

impl fmt::Display for TreeDragSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeDragSource")
    }
}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Action, Component, Document, EditableText, Hypertext, Image, Object, Selection, Table,
    TableCell, Text, Value, Window,
};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkNoOpObject")]
    pub struct NoOpObject(Object<ffi::AtkNoOpObject, ffi::AtkNoOpObjectClass>) @extends Object, @implements Action, Component, Document, EditableText, Hypertext, Image, Selection, Table, TableCell, Text, Value, Window;

    match fn {
        type_ => || ffi::atk_no_op_object_get_type(),
    }
}

impl NoOpObject {
    pub const NONE: Option<&'static NoOpObject> = None;

    #[doc(alias = "atk_no_op_object_new")]
    pub fn new(obj: &impl IsA<glib::Object>) -> NoOpObject {
        assert_initialized_main_thread!();
        unsafe {
            Object::from_glib_full(ffi::atk_no_op_object_new(obj.as_ref().to_glib_none().0))
                .unsafe_cast()
        }
    }
}

impl fmt::Display for NoOpObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NoOpObject")
    }
}
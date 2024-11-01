// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SessionFeature;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SoupContentDecoder")]
    pub struct ContentDecoder(Object<ffi::SoupContentDecoder, ffi::SoupContentDecoderClass>) @implements SessionFeature;

    match fn {
        type_ => || ffi::soup_content_decoder_get_type(),
    }
}

impl ContentDecoder {}

impl fmt::Display for ContentDecoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContentDecoder")
    }
}

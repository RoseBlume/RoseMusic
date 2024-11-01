// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Cookie, DateFormat, HTTPVersion, Message, MessageHeaders};
use glib::translate::*;
use std::{mem, ptr};

#[doc(alias = "soup_check_version")]
pub fn check_version(major: u32, minor: u32, micro: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::soup_check_version(major, minor, micro)) }
}

#[doc(alias = "soup_cookies_from_request")]
pub fn cookies_from_request(msg: &Message) -> Vec<Cookie> {
    skip_assert_initialized!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::soup_cookies_from_request(msg.to_glib_none().0))
    }
}

#[doc(alias = "soup_cookies_from_response")]
pub fn cookies_from_response(msg: &Message) -> Vec<Cookie> {
    skip_assert_initialized!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::soup_cookies_from_response(msg.to_glib_none().0))
    }
}

#[doc(alias = "soup_date_time_new_from_http_string")]
pub fn date_time_new_from_http_string(date_string: &str) -> Option<glib::DateTime> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::soup_date_time_new_from_http_string(
            date_string.to_glib_none().0,
        ))
    }
}

#[doc(alias = "soup_date_time_to_string")]
pub fn date_time_to_string(date: &glib::DateTime, format: DateFormat) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::soup_date_time_to_string(
            date.to_glib_none().0,
            format.into_glib(),
        ))
    }
}

//#[doc(alias = "soup_form_decode")]
//pub fn form_decode(encoded_form: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_form_decode() }
//}

//#[doc(alias = "soup_form_decode_multipart")]
//pub fn form_decode_multipart(multipart: &mut Multipart, file_control_name: Option<&str>) -> (/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, glib::GString, glib::GString, glib::Bytes) {
//    unsafe { TODO: call ffi:soup_form_decode_multipart() }
//}

//#[doc(alias = "soup_form_encode")]
//pub fn form_encode(first_field: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode() }
//}

//#[doc(alias = "soup_form_encode_datalist")]
//pub fn form_encode_datalist(form_data_set: /*Ignored*/&mut glib::Data) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode_datalist() }
//}

//#[doc(alias = "soup_form_encode_hash")]
//pub fn form_encode_hash(form_data_set: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode_hash() }
//}

//#[doc(alias = "soup_form_encode_valist")]
//pub fn form_encode_valist(first_field: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode_valist() }
//}

#[doc(alias = "soup_get_major_version")]
#[doc(alias = "get_major_version")]
pub fn major_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::soup_get_major_version() }
}

#[doc(alias = "soup_get_micro_version")]
#[doc(alias = "get_micro_version")]
pub fn micro_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::soup_get_micro_version() }
}

#[doc(alias = "soup_get_minor_version")]
#[doc(alias = "get_minor_version")]
pub fn minor_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::soup_get_minor_version() }
}

#[doc(alias = "soup_header_contains")]
pub fn header_contains(header: &str, token: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_header_contains(
            header.to_glib_none().0,
            token.to_glib_none().0,
        ))
    }
}

//#[doc(alias = "soup_header_free_list")]
//pub fn header_free_list(list: /*Unimplemented*/&[&Basic: Pointer]) {
//    unsafe { TODO: call ffi:soup_header_free_list() }
//}

//#[doc(alias = "soup_header_free_param_list")]
//pub fn header_free_param_list(param_list: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
//    unsafe { TODO: call ffi:soup_header_free_param_list() }
//}

//#[doc(alias = "soup_header_g_string_append_param")]
//pub fn header_g_string_append_param(string: /*Ignored*/&mut glib::String, name: &str, value: Option<&str>) {
//    unsafe { TODO: call ffi:soup_header_g_string_append_param() }
//}

//#[doc(alias = "soup_header_g_string_append_param_quoted")]
//pub fn header_g_string_append_param_quoted(string: /*Ignored*/&mut glib::String, name: &str, value: &str) {
//    unsafe { TODO: call ffi:soup_header_g_string_append_param_quoted() }
//}

#[doc(alias = "soup_header_parse_list")]
pub fn header_parse_list(header: &str) -> Vec<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::soup_header_parse_list(header.to_glib_none().0))
    }
}

//#[doc(alias = "soup_header_parse_param_list")]
//pub fn header_parse_param_list(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_param_list() }
//}

//#[doc(alias = "soup_header_parse_param_list_strict")]
//pub fn header_parse_param_list_strict(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_param_list_strict() }
//}

//#[doc(alias = "soup_header_parse_quality_list")]
//pub fn header_parse_quality_list(header: &str, unacceptable: /*Unimplemented*/Vec<glib::GString>) -> Vec<glib::GString> {
//    unsafe { TODO: call ffi:soup_header_parse_quality_list() }
//}

//#[doc(alias = "soup_header_parse_semi_param_list")]
//pub fn header_parse_semi_param_list(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_semi_param_list() }
//}

//#[doc(alias = "soup_header_parse_semi_param_list_strict")]
//pub fn header_parse_semi_param_list_strict(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_semi_param_list_strict() }
//}

#[doc(alias = "soup_headers_parse")]
pub fn headers_parse(str: &str, dest: &MessageHeaders) -> bool {
    assert_initialized_main_thread!();
    let len = str.len() as _;
    unsafe {
        from_glib(ffi::soup_headers_parse(
            str.to_glib_none().0,
            len,
            dest.to_glib_none().0,
        ))
    }
}

#[doc(alias = "soup_headers_parse_request")]
pub fn headers_parse_request(
    str: &str,
    req_headers: &MessageHeaders,
) -> (u32, glib::GString, glib::GString, HTTPVersion) {
    assert_initialized_main_thread!();
    let len = str.len() as _;
    unsafe {
        let mut req_method = ptr::null_mut();
        let mut req_path = ptr::null_mut();
        let mut ver = mem::MaybeUninit::uninit();
        let ret = ffi::soup_headers_parse_request(
            str.to_glib_none().0,
            len,
            req_headers.to_glib_none().0,
            &mut req_method,
            &mut req_path,
            ver.as_mut_ptr(),
        );
        (
            ret,
            from_glib_full(req_method),
            from_glib_full(req_path),
            from_glib(ver.assume_init()),
        )
    }
}

#[doc(alias = "soup_headers_parse_response")]
pub fn headers_parse_response(
    str: &str,
    headers: &MessageHeaders,
) -> Option<(HTTPVersion, u32, glib::GString)> {
    assert_initialized_main_thread!();
    let len = str.len() as _;
    unsafe {
        let mut ver = mem::MaybeUninit::uninit();
        let mut status_code = mem::MaybeUninit::uninit();
        let mut reason_phrase = ptr::null_mut();
        let ret = from_glib(ffi::soup_headers_parse_response(
            str.to_glib_none().0,
            len,
            headers.to_glib_none().0,
            ver.as_mut_ptr(),
            status_code.as_mut_ptr(),
            &mut reason_phrase,
        ));
        if ret {
            Some((
                from_glib(ver.assume_init()),
                status_code.assume_init(),
                from_glib_full(reason_phrase),
            ))
        } else {
            None
        }
    }
}

#[doc(alias = "soup_headers_parse_status_line")]
pub fn headers_parse_status_line(status_line: &str) -> Option<(HTTPVersion, u32, glib::GString)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut ver = mem::MaybeUninit::uninit();
        let mut status_code = mem::MaybeUninit::uninit();
        let mut reason_phrase = ptr::null_mut();
        let ret = from_glib(ffi::soup_headers_parse_status_line(
            status_line.to_glib_none().0,
            ver.as_mut_ptr(),
            status_code.as_mut_ptr(),
            &mut reason_phrase,
        ));
        if ret {
            Some((
                from_glib(ver.assume_init()),
                status_code.assume_init(),
                from_glib_full(reason_phrase),
            ))
        } else {
            None
        }
    }
}

#[doc(alias = "soup_tld_domain_is_public_suffix")]
pub fn tld_domain_is_public_suffix(domain: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_tld_domain_is_public_suffix(
            domain.to_glib_none().0,
        ))
    }
}

#[doc(alias = "soup_tld_get_base_domain")]
pub fn tld_get_base_domain(hostname: &str) -> Result<glib::GString, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_tld_get_base_domain(hostname.to_glib_none().0, &mut error);
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

//#[doc(alias = "soup_uri_copy")]
//pub fn uri_copy(uri: &glib::Uri, first_component: URIComponent, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<glib::Uri> {
//    unsafe { TODO: call ffi:soup_uri_copy() }
//}

#[doc(alias = "soup_uri_decode_data_uri")]
pub fn uri_decode_data_uri(uri: &str) -> (glib::Bytes, Option<glib::GString>) {
    assert_initialized_main_thread!();
    unsafe {
        let mut content_type = ptr::null_mut();
        let ret = from_glib_full(ffi::soup_uri_decode_data_uri(
            uri.to_glib_none().0,
            &mut content_type,
        ));
        (ret, from_glib_full(content_type))
    }
}

#[doc(alias = "soup_uri_equal")]
pub fn uri_equal(uri1: &glib::Uri, uri2: &glib::Uri) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_uri_equal(
            uri1.to_glib_none().0,
            uri2.to_glib_none().0,
        ))
    }
}

//#[doc(alias = "soup_websocket_client_prepare_handshake")]
//pub fn websocket_client_prepare_handshake(msg: &Message, origin: Option<&str>, protocols: &[&str], supported_extensions: /*Ignored*/&[&glib::TypeClass]) {
//    unsafe { TODO: call ffi:soup_websocket_client_prepare_handshake() }
//}

//#[doc(alias = "soup_websocket_client_verify_handshake")]
//pub fn websocket_client_verify_handshake(msg: &Message, supported_extensions: /*Ignored*/&[&glib::TypeClass], accepted_extensions: /*Unimplemented*/Vec<WebsocketExtension>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:soup_websocket_client_verify_handshake() }
//}

//#[doc(alias = "soup_websocket_server_check_handshake")]
//pub fn websocket_server_check_handshake(msg: &ServerMessage, origin: Option<&str>, protocols: &[&str], supported_extensions: /*Ignored*/&[&glib::TypeClass]) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:soup_websocket_server_check_handshake() }
//}

//#[doc(alias = "soup_websocket_server_process_handshake")]
//pub fn websocket_server_process_handshake(msg: &ServerMessage, expected_origin: Option<&str>, protocols: &[&str], supported_extensions: /*Ignored*/&[&glib::TypeClass], accepted_extensions: /*Unimplemented*/Vec<WebsocketExtension>) -> bool {
//    unsafe { TODO: call ffi:soup_websocket_server_process_handshake() }
//}

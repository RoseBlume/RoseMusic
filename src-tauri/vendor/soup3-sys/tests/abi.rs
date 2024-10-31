// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![cfg(unix)]

use soup3_sys::*;
use std::mem::{align_of, size_of};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["libsoup-3.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG")
        .unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {cmd:?} returned {}", out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in
        RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {abi_cmd:?} failed, {output:?}").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("SoupAuth", Layout {size: size_of::<SoupAuth>(), alignment: align_of::<SoupAuth>()}),
    ("SoupAuthClass", Layout {size: size_of::<SoupAuthClass>(), alignment: align_of::<SoupAuthClass>()}),
    ("SoupAuthDomain", Layout {size: size_of::<SoupAuthDomain>(), alignment: align_of::<SoupAuthDomain>()}),
    ("SoupAuthDomainBasicClass", Layout {size: size_of::<SoupAuthDomainBasicClass>(), alignment: align_of::<SoupAuthDomainBasicClass>()}),
    ("SoupAuthDomainClass", Layout {size: size_of::<SoupAuthDomainClass>(), alignment: align_of::<SoupAuthDomainClass>()}),
    ("SoupAuthDomainDigestClass", Layout {size: size_of::<SoupAuthDomainDigestClass>(), alignment: align_of::<SoupAuthDomainDigestClass>()}),
    ("SoupAuthManagerClass", Layout {size: size_of::<SoupAuthManagerClass>(), alignment: align_of::<SoupAuthManagerClass>()}),
    ("SoupCache", Layout {size: size_of::<SoupCache>(), alignment: align_of::<SoupCache>()}),
    ("SoupCacheClass", Layout {size: size_of::<SoupCacheClass>(), alignment: align_of::<SoupCacheClass>()}),
    ("SoupCacheType", Layout {size: size_of::<SoupCacheType>(), alignment: align_of::<SoupCacheType>()}),
    ("SoupCacheability", Layout {size: size_of::<SoupCacheability>(), alignment: align_of::<SoupCacheability>()}),
    ("SoupContentDecoderClass", Layout {size: size_of::<SoupContentDecoderClass>(), alignment: align_of::<SoupContentDecoderClass>()}),
    ("SoupContentSnifferClass", Layout {size: size_of::<SoupContentSnifferClass>(), alignment: align_of::<SoupContentSnifferClass>()}),
    ("SoupCookieJar", Layout {size: size_of::<SoupCookieJar>(), alignment: align_of::<SoupCookieJar>()}),
    ("SoupCookieJarAcceptPolicy", Layout {size: size_of::<SoupCookieJarAcceptPolicy>(), alignment: align_of::<SoupCookieJarAcceptPolicy>()}),
    ("SoupCookieJarClass", Layout {size: size_of::<SoupCookieJarClass>(), alignment: align_of::<SoupCookieJarClass>()}),
    ("SoupCookieJarDBClass", Layout {size: size_of::<SoupCookieJarDBClass>(), alignment: align_of::<SoupCookieJarDBClass>()}),
    ("SoupCookieJarTextClass", Layout {size: size_of::<SoupCookieJarTextClass>(), alignment: align_of::<SoupCookieJarTextClass>()}),
    ("SoupDateFormat", Layout {size: size_of::<SoupDateFormat>(), alignment: align_of::<SoupDateFormat>()}),
    ("SoupEncoding", Layout {size: size_of::<SoupEncoding>(), alignment: align_of::<SoupEncoding>()}),
    ("SoupExpectation", Layout {size: size_of::<SoupExpectation>(), alignment: align_of::<SoupExpectation>()}),
    ("SoupHSTSEnforcer", Layout {size: size_of::<SoupHSTSEnforcer>(), alignment: align_of::<SoupHSTSEnforcer>()}),
    ("SoupHSTSEnforcerClass", Layout {size: size_of::<SoupHSTSEnforcerClass>(), alignment: align_of::<SoupHSTSEnforcerClass>()}),
    ("SoupHSTSEnforcerDBClass", Layout {size: size_of::<SoupHSTSEnforcerDBClass>(), alignment: align_of::<SoupHSTSEnforcerDBClass>()}),
    ("SoupHTTPVersion", Layout {size: size_of::<SoupHTTPVersion>(), alignment: align_of::<SoupHTTPVersion>()}),
    ("SoupLoggerClass", Layout {size: size_of::<SoupLoggerClass>(), alignment: align_of::<SoupLoggerClass>()}),
    ("SoupLoggerLogLevel", Layout {size: size_of::<SoupLoggerLogLevel>(), alignment: align_of::<SoupLoggerLogLevel>()}),
    ("SoupMemoryUse", Layout {size: size_of::<SoupMemoryUse>(), alignment: align_of::<SoupMemoryUse>()}),
    ("SoupMessageBody", Layout {size: size_of::<SoupMessageBody>(), alignment: align_of::<SoupMessageBody>()}),
    ("SoupMessageClass", Layout {size: size_of::<SoupMessageClass>(), alignment: align_of::<SoupMessageClass>()}),
    ("SoupMessageFlags", Layout {size: size_of::<SoupMessageFlags>(), alignment: align_of::<SoupMessageFlags>()}),
    ("SoupMessageHeadersIter", Layout {size: size_of::<SoupMessageHeadersIter>(), alignment: align_of::<SoupMessageHeadersIter>()}),
    ("SoupMessageHeadersType", Layout {size: size_of::<SoupMessageHeadersType>(), alignment: align_of::<SoupMessageHeadersType>()}),
    ("SoupMessagePriority", Layout {size: size_of::<SoupMessagePriority>(), alignment: align_of::<SoupMessagePriority>()}),
    ("SoupMultipartInputStreamClass", Layout {size: size_of::<SoupMultipartInputStreamClass>(), alignment: align_of::<SoupMultipartInputStreamClass>()}),
    ("SoupRange", Layout {size: size_of::<SoupRange>(), alignment: align_of::<SoupRange>()}),
    ("SoupSameSitePolicy", Layout {size: size_of::<SoupSameSitePolicy>(), alignment: align_of::<SoupSameSitePolicy>()}),
    ("SoupServer", Layout {size: size_of::<SoupServer>(), alignment: align_of::<SoupServer>()}),
    ("SoupServerClass", Layout {size: size_of::<SoupServerClass>(), alignment: align_of::<SoupServerClass>()}),
    ("SoupServerListenOptions", Layout {size: size_of::<SoupServerListenOptions>(), alignment: align_of::<SoupServerListenOptions>()}),
    ("SoupServerMessageClass", Layout {size: size_of::<SoupServerMessageClass>(), alignment: align_of::<SoupServerMessageClass>()}),
    ("SoupSession", Layout {size: size_of::<SoupSession>(), alignment: align_of::<SoupSession>()}),
    ("SoupSessionClass", Layout {size: size_of::<SoupSessionClass>(), alignment: align_of::<SoupSessionClass>()}),
    ("SoupSessionError", Layout {size: size_of::<SoupSessionError>(), alignment: align_of::<SoupSessionError>()}),
    ("SoupStatus", Layout {size: size_of::<SoupStatus>(), alignment: align_of::<SoupStatus>()}),
    ("SoupTLDError", Layout {size: size_of::<SoupTLDError>(), alignment: align_of::<SoupTLDError>()}),
    ("SoupURIComponent", Layout {size: size_of::<SoupURIComponent>(), alignment: align_of::<SoupURIComponent>()}),
    ("SoupWebsocketCloseCode", Layout {size: size_of::<SoupWebsocketCloseCode>(), alignment: align_of::<SoupWebsocketCloseCode>()}),
    ("SoupWebsocketConnectionClass", Layout {size: size_of::<SoupWebsocketConnectionClass>(), alignment: align_of::<SoupWebsocketConnectionClass>()}),
    ("SoupWebsocketConnectionType", Layout {size: size_of::<SoupWebsocketConnectionType>(), alignment: align_of::<SoupWebsocketConnectionType>()}),
    ("SoupWebsocketDataType", Layout {size: size_of::<SoupWebsocketDataType>(), alignment: align_of::<SoupWebsocketDataType>()}),
    ("SoupWebsocketError", Layout {size: size_of::<SoupWebsocketError>(), alignment: align_of::<SoupWebsocketError>()}),
    ("SoupWebsocketExtension", Layout {size: size_of::<SoupWebsocketExtension>(), alignment: align_of::<SoupWebsocketExtension>()}),
    ("SoupWebsocketExtensionClass", Layout {size: size_of::<SoupWebsocketExtensionClass>(), alignment: align_of::<SoupWebsocketExtensionClass>()}),
    ("SoupWebsocketExtensionDeflateClass", Layout {size: size_of::<SoupWebsocketExtensionDeflateClass>(), alignment: align_of::<SoupWebsocketExtensionDeflateClass>()}),
    ("SoupWebsocketExtensionManagerClass", Layout {size: size_of::<SoupWebsocketExtensionManagerClass>(), alignment: align_of::<SoupWebsocketExtensionManagerClass>()}),
    ("SoupWebsocketState", Layout {size: size_of::<SoupWebsocketState>(), alignment: align_of::<SoupWebsocketState>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(guint) SOUP_CACHE_CACHEABLE", "1"),
    ("(guint) SOUP_CACHE_INVALIDATES", "4"),
    ("(gint) SOUP_CACHE_SHARED", "1"),
    ("(gint) SOUP_CACHE_SINGLE_USER", "0"),
    ("(guint) SOUP_CACHE_UNCACHEABLE", "2"),
    ("(guint) SOUP_CACHE_VALIDATES", "8"),
    ("(gint) SOUP_COOKIE_JAR_ACCEPT_ALWAYS", "0"),
    ("(gint) SOUP_COOKIE_JAR_ACCEPT_GRANDFATHERED_THIRD_PARTY", "3"),
    ("(gint) SOUP_COOKIE_JAR_ACCEPT_NEVER", "1"),
    ("(gint) SOUP_COOKIE_JAR_ACCEPT_NO_THIRD_PARTY", "2"),
    ("SOUP_COOKIE_MAX_AGE_ONE_DAY", "0"),
    ("SOUP_COOKIE_MAX_AGE_ONE_HOUR", "3600"),
    ("SOUP_COOKIE_MAX_AGE_ONE_WEEK", "0"),
    ("SOUP_COOKIE_MAX_AGE_ONE_YEAR", "0"),
    ("(gint) SOUP_DATE_COOKIE", "2"),
    ("(gint) SOUP_DATE_HTTP", "1"),
    ("(gint) SOUP_ENCODING_BYTERANGES", "5"),
    ("(gint) SOUP_ENCODING_CHUNKED", "4"),
    ("(gint) SOUP_ENCODING_CONTENT_LENGTH", "2"),
    ("(gint) SOUP_ENCODING_EOF", "3"),
    ("(gint) SOUP_ENCODING_NONE", "1"),
    ("(gint) SOUP_ENCODING_UNRECOGNIZED", "0"),
    ("(guint) SOUP_EXPECTATION_CONTINUE", "2"),
    ("(guint) SOUP_EXPECTATION_UNRECOGNIZED", "1"),
    ("SOUP_FORM_MIME_TYPE_MULTIPART", "multipart/form-data"),
    ("SOUP_FORM_MIME_TYPE_URLENCODED", "application/x-www-form-urlencoded"),
    ("SOUP_HSTS_POLICY_MAX_AGE_PAST", "0"),
    ("(gint) SOUP_HTTP_1_0", "0"),
    ("(gint) SOUP_HTTP_1_1", "1"),
    ("(gint) SOUP_HTTP_2_0", "2"),
    ("SOUP_HTTP_URI_FLAGS", "482"),
    ("(gint) SOUP_LOGGER_LOG_BODY", "3"),
    ("(gint) SOUP_LOGGER_LOG_HEADERS", "2"),
    ("(gint) SOUP_LOGGER_LOG_MINIMAL", "1"),
    ("(gint) SOUP_LOGGER_LOG_NONE", "0"),
    ("SOUP_MAJOR_VERSION", "3"),
    ("(gint) SOUP_MEMORY_COPY", "2"),
    ("(gint) SOUP_MEMORY_STATIC", "0"),
    ("(gint) SOUP_MEMORY_TAKE", "1"),
    ("(guint) SOUP_MESSAGE_COLLECT_METRICS", "32"),
    ("(guint) SOUP_MESSAGE_DO_NOT_USE_AUTH_CACHE", "16"),
    ("(gint) SOUP_MESSAGE_HEADERS_MULTIPART", "2"),
    ("(gint) SOUP_MESSAGE_HEADERS_REQUEST", "0"),
    ("(gint) SOUP_MESSAGE_HEADERS_RESPONSE", "1"),
    ("(guint) SOUP_MESSAGE_IDEMPOTENT", "8"),
    ("(guint) SOUP_MESSAGE_NEW_CONNECTION", "4"),
    ("(guint) SOUP_MESSAGE_NO_REDIRECT", "2"),
    ("(gint) SOUP_MESSAGE_PRIORITY_HIGH", "3"),
    ("(gint) SOUP_MESSAGE_PRIORITY_LOW", "1"),
    ("(gint) SOUP_MESSAGE_PRIORITY_NORMAL", "2"),
    ("(gint) SOUP_MESSAGE_PRIORITY_VERY_HIGH", "4"),
    ("(gint) SOUP_MESSAGE_PRIORITY_VERY_LOW", "0"),
    ("(gint) SOUP_SAME_SITE_POLICY_LAX", "1"),
    ("(gint) SOUP_SAME_SITE_POLICY_NONE", "0"),
    ("(gint) SOUP_SAME_SITE_POLICY_STRICT", "2"),
    ("(guint) SOUP_SERVER_LISTEN_HTTPS", "1"),
    ("(guint) SOUP_SERVER_LISTEN_IPV4_ONLY", "2"),
    ("(guint) SOUP_SERVER_LISTEN_IPV6_ONLY", "4"),
    ("(gint) SOUP_SESSION_ERROR_ENCODING", "1"),
    ("(gint) SOUP_SESSION_ERROR_MESSAGE_ALREADY_IN_QUEUE", "6"),
    ("(gint) SOUP_SESSION_ERROR_PARSING", "0"),
    ("(gint) SOUP_SESSION_ERROR_REDIRECT_BAD_URI", "5"),
    ("(gint) SOUP_SESSION_ERROR_REDIRECT_NO_LOCATION", "4"),
    ("(gint) SOUP_SESSION_ERROR_TOO_MANY_REDIRECTS", "2"),
    ("(gint) SOUP_SESSION_ERROR_TOO_MANY_RESTARTS", "3"),
    ("(gint) SOUP_STATUS_ACCEPTED", "202"),
    ("(gint) SOUP_STATUS_BAD_GATEWAY", "502"),
    ("(gint) SOUP_STATUS_BAD_REQUEST", "400"),
    ("(gint) SOUP_STATUS_CONFLICT", "409"),
    ("(gint) SOUP_STATUS_CONTINUE", "100"),
    ("(gint) SOUP_STATUS_CREATED", "201"),
    ("(gint) SOUP_STATUS_EXPECTATION_FAILED", "417"),
    ("(gint) SOUP_STATUS_FAILED_DEPENDENCY", "424"),
    ("(gint) SOUP_STATUS_FORBIDDEN", "403"),
    ("(gint) SOUP_STATUS_FOUND", "302"),
    ("(gint) SOUP_STATUS_GATEWAY_TIMEOUT", "504"),
    ("(gint) SOUP_STATUS_GONE", "410"),
    ("(gint) SOUP_STATUS_HTTP_VERSION_NOT_SUPPORTED", "505"),
    ("(gint) SOUP_STATUS_INSUFFICIENT_STORAGE", "507"),
    ("(gint) SOUP_STATUS_INTERNAL_SERVER_ERROR", "500"),
    ("(gint) SOUP_STATUS_INVALID_RANGE", "416"),
    ("(gint) SOUP_STATUS_LENGTH_REQUIRED", "411"),
    ("(gint) SOUP_STATUS_LOCKED", "423"),
    ("(gint) SOUP_STATUS_METHOD_NOT_ALLOWED", "405"),
    ("(gint) SOUP_STATUS_MISDIRECTED_REQUEST", "421"),
    ("(gint) SOUP_STATUS_MOVED_PERMANENTLY", "301"),
    ("(gint) SOUP_STATUS_MOVED_TEMPORARILY", "302"),
    ("(gint) SOUP_STATUS_MULTIPLE_CHOICES", "300"),
    ("(gint) SOUP_STATUS_MULTI_STATUS", "207"),
    ("(gint) SOUP_STATUS_NONE", "0"),
    ("(gint) SOUP_STATUS_NON_AUTHORITATIVE", "203"),
    ("(gint) SOUP_STATUS_NOT_ACCEPTABLE", "406"),
    ("(gint) SOUP_STATUS_NOT_APPEARING_IN_THIS_PROTOCOL", "306"),
    ("(gint) SOUP_STATUS_NOT_EXTENDED", "510"),
    ("(gint) SOUP_STATUS_NOT_FOUND", "404"),
    ("(gint) SOUP_STATUS_NOT_IMPLEMENTED", "501"),
    ("(gint) SOUP_STATUS_NOT_MODIFIED", "304"),
    ("(gint) SOUP_STATUS_NO_CONTENT", "204"),
    ("(gint) SOUP_STATUS_OK", "200"),
    ("(gint) SOUP_STATUS_PARTIAL_CONTENT", "206"),
    ("(gint) SOUP_STATUS_PAYMENT_REQUIRED", "402"),
    ("(gint) SOUP_STATUS_PERMANENT_REDIRECT", "308"),
    ("(gint) SOUP_STATUS_PRECONDITION_FAILED", "412"),
    ("(gint) SOUP_STATUS_PROCESSING", "102"),
    ("(gint) SOUP_STATUS_PROXY_AUTHENTICATION_REQUIRED", "407"),
    ("(gint) SOUP_STATUS_PROXY_UNAUTHORIZED", "407"),
    ("(gint) SOUP_STATUS_REQUESTED_RANGE_NOT_SATISFIABLE", "416"),
    ("(gint) SOUP_STATUS_REQUEST_ENTITY_TOO_LARGE", "413"),
    ("(gint) SOUP_STATUS_REQUEST_TIMEOUT", "408"),
    ("(gint) SOUP_STATUS_REQUEST_URI_TOO_LONG", "414"),
    ("(gint) SOUP_STATUS_RESET_CONTENT", "205"),
    ("(gint) SOUP_STATUS_SEE_OTHER", "303"),
    ("(gint) SOUP_STATUS_SERVICE_UNAVAILABLE", "503"),
    ("(gint) SOUP_STATUS_SWITCHING_PROTOCOLS", "101"),
    ("(gint) SOUP_STATUS_TEMPORARY_REDIRECT", "307"),
    ("(gint) SOUP_STATUS_UNAUTHORIZED", "401"),
    ("(gint) SOUP_STATUS_UNPROCESSABLE_ENTITY", "422"),
    ("(gint) SOUP_STATUS_UNSUPPORTED_MEDIA_TYPE", "415"),
    ("(gint) SOUP_STATUS_USE_PROXY", "305"),
    ("(gint) SOUP_TLD_ERROR_INVALID_HOSTNAME", "0"),
    ("(gint) SOUP_TLD_ERROR_IS_IP_ADDRESS", "1"),
    ("(gint) SOUP_TLD_ERROR_NOT_ENOUGH_DOMAINS", "2"),
    ("(gint) SOUP_TLD_ERROR_NO_BASE_DOMAIN", "3"),
    ("(gint) SOUP_TLD_ERROR_NO_PSL_DATA", "4"),
    ("(gint) SOUP_URI_AUTH_PARAMS", "4"),
    ("(gint) SOUP_URI_FRAGMENT", "9"),
    ("(gint) SOUP_URI_HOST", "5"),
    ("(gint) SOUP_URI_NONE", "0"),
    ("(gint) SOUP_URI_PASSWORD", "3"),
    ("(gint) SOUP_URI_PATH", "7"),
    ("(gint) SOUP_URI_PORT", "6"),
    ("(gint) SOUP_URI_QUERY", "8"),
    ("(gint) SOUP_URI_SCHEME", "1"),
    ("(gint) SOUP_URI_USER", "2"),
    ("SOUP_VERSION_MIN_REQUIRED", "2"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_ABNORMAL", "1006"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_BAD_DATA", "1007"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_GOING_AWAY", "1001"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_NORMAL", "1000"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_NO_EXTENSION", "1010"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_NO_STATUS", "1005"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_POLICY_VIOLATION", "1008"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_PROTOCOL_ERROR", "1002"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_SERVER_ERROR", "1011"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_TLS_HANDSHAKE", "1015"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_TOO_BIG", "1009"),
    ("(gint) SOUP_WEBSOCKET_CLOSE_UNSUPPORTED_DATA", "1003"),
    ("(gint) SOUP_WEBSOCKET_CONNECTION_CLIENT", "1"),
    ("(gint) SOUP_WEBSOCKET_CONNECTION_SERVER", "2"),
    ("(gint) SOUP_WEBSOCKET_CONNECTION_UNKNOWN", "0"),
    ("(gint) SOUP_WEBSOCKET_DATA_BINARY", "2"),
    ("(gint) SOUP_WEBSOCKET_DATA_TEXT", "1"),
    ("(gint) SOUP_WEBSOCKET_ERROR_BAD_HANDSHAKE", "2"),
    ("(gint) SOUP_WEBSOCKET_ERROR_BAD_ORIGIN", "3"),
    ("(gint) SOUP_WEBSOCKET_ERROR_FAILED", "0"),
    ("(gint) SOUP_WEBSOCKET_ERROR_NOT_WEBSOCKET", "1"),
    ("(gint) SOUP_WEBSOCKET_STATE_CLOSED", "3"),
    ("(gint) SOUP_WEBSOCKET_STATE_CLOSING", "2"),
    ("(gint) SOUP_WEBSOCKET_STATE_OPEN", "1"),
];


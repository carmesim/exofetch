use std::{ffi::OsString, mem::MaybeUninit, sync::Once};

use cala_core::os::web::{JsFn, JsString};

use crate::{DesktopEnv, Platform};

static mut USER_AGENT: MaybeUninit<JsFn> = MaybeUninit::uninit();
static INIT_USER_AGENT: Once = Once::new();

static mut DOCUMENT_DOMAIN: MaybeUninit<JsFn> = MaybeUninit::uninit();
static INIT_DOCUMENT_DOMAIN: Once = Once::new();

// Get the user agent
fn user_agent() -> String {
    unsafe {
        INIT_USER_AGENT.call_once(|| {
            USER_AGENT =
                MaybeUninit::new(JsFn::new("return navigator.userAgent;"));
        });
        let user_agent = &*USER_AGENT.as_ptr();
        let string = JsString::from_var(user_agent.call(None, None).unwrap());
        let mut text = Vec::new();
        string.as_var().read_utf16(&mut text);
        String::from_utf16_lossy(&text)
    }
}

// Get the document domain
fn document_domain() -> String {
    unsafe {
        INIT_DOCUMENT_DOMAIN.call_once(|| {
            DOCUMENT_DOMAIN =
                MaybeUninit::new(JsFn::new("return document.domain;"));
        });
        let domain = &*DOCUMENT_DOMAIN.as_ptr();
        let string = JsString::from_var(domain.call(None, None).unwrap());
        let mut text = Vec::new();
        string.as_var().read_utf16(&mut text);
        String::from_utf16_lossy(&text)
    }
}

#[inline(always)]
pub fn username_os() -> OsString {
    username().into()
}

#[inline(always)]
pub fn realname_os() -> OsString {
    realname().into()
}

#[inline(always)]
pub fn devicename_os() -> OsString {
    devicename().into()
}

#[inline(always)]
pub fn hostname_os() -> OsString {
    hostname().into()
}

#[inline(always)]
pub fn distro_os() -> Option<OsString> {
    distro().map(|a| a.into())
}

#[inline(always)]
pub fn username() -> String {
    "anonymous".to_string()
}

#[inline(always)]
pub fn realname() -> String {
    "Anonymous".to_string()
}

pub fn devicename() -> String {
    let orig_string = user_agent();

    let start = if let Some(s) = orig_string.rfind(" ") {
        s
    } else {
        return "Unknown Browser".to_string();
    };

    let string = orig_string
        .get(start + 1..)
        .unwrap_or("Unknown Browser")
        .replace('/', " ");

    if string == "Safari" {
        if orig_string.contains("Chrome") {
            "Chrome".to_string()
        } else {
            "Safari".to_string()
        }
    } else {
        string
    }
}

#[inline(always)]
pub fn hostname() -> String {
    document_domain()
}

pub fn distro() -> Option<String> {
    let string = user_agent();

    let begin = if let Some(b) = string.find('(') {
        b
    } else {
        return None;
    };
    let end = if let Some(e) = string.find(')') {
        e
    } else {
        return None;
    };
    let string = &string[begin + 1..end];

    if string.contains("Win32") || string.contains("Win64") {
        let begin = if let Some(b) = string.find("NT") {
            b
        } else {
            return Some("Windows".to_string());
        };
        let end = if let Some(e) = string.find(".") {
            e
        } else {
            return Some("Windows".to_string());
        };
        let string = &string[begin + 3..end];

        Some(format!("Windows {}", string))
    } else if string.contains("Linux") {
        let string = if string.contains("X11") || string.contains("Wayland") {
            let begin = if let Some(b) = string.find(";") {
                b
            } else {
                return Some("Unknown Linux".to_string());
            };
            let string = &string[begin + 2..];

            string
        } else {
            string
        };

        if string.starts_with("Linux") {
            Some("Unknown Linux".to_string())
        } else {
            let end = if let Some(e) = string.find(";") {
                e
            } else {
                return Some("Unknown Linux".to_string());
            };
            Some(string[..end].to_string())
        }
    } else if string.contains("Mac OS X") {
        let begin = string.find("Mac OS X").unwrap();
        Some(if let Some(end) = string[begin..].find(";") {
            string[begin..begin + end].to_string()
        } else {
            string[begin..].to_string().replace("_", ".")
        })
    } else {
        // TODO:
        // Platform::FreeBsd,
        // Platform::Ios,
        // Platform::Android,
        // Platform::Nintendo,
        // Platform::Xbox,
        // Platform::PlayStation,
        // Platform::Dive,
        // Platform::Fuchsia,
        // Platform::Redox
        Some(string.to_string())
    }
}

pub const fn desktop_env() -> DesktopEnv {
    DesktopEnv::WebBrowser
}

pub fn platform() -> Platform {
    let string = user_agent();

    let begin = if let Some(b) = string.find('(') {
        b
    } else {
        return Platform::Unknown("Unknown".to_string());
    };
    let end = if let Some(e) = string.find(')') {
        e
    } else {
        return Platform::Unknown("Unknown".to_string());
    };
    let string = &string[begin + 1..end];

    if string.contains("Win32") || string.contains("Win64") {
        Platform::Windows
    } else if string.contains("Linux") {
        Platform::Linux
    } else if string.contains("Mac OS X") {
        Platform::MacOS
    } else {
        // TODO:
        // Platform::FreeBsd,
        // Platform::Ios,
        // Platform::Android,
        // Platform::Nintendo,
        // Platform::Xbox,
        // Platform::PlayStation,
        // Platform::Dive,
        // Platform::Fuchsia,
        // Platform::Redox,
        Platform::Unknown(string.to_string())
    }
}

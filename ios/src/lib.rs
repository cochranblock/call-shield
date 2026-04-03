//! Call Shield iOS static library.
//! Exposes classification and screening logic to Swift via C ABI.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Classify a transcript string. Returns a C string: "VERDICT|score|matched"
/// Caller must free the result with `call_shield_free`.
///
/// # Safety
/// `transcript` must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_shield_classify(transcript: *const c_char) -> *mut c_char {
    if transcript.is_null() {
        let empty = CString::new("UNKNOWN|0.50|").unwrap();
        return empty.into_raw();
    }

    let text = unsafe { CStr::from_ptr(transcript) }
        .to_string_lossy()
        .to_lowercase();

    let (verdict, score, matched) = classify(&text);
    let result = format!("{verdict}|{score:.2}|{matched}");
    CString::new(result).unwrap().into_raw()
}

/// Free a string returned by `call_shield_classify`.
///
/// # Safety
/// `ptr` must have been returned by `call_shield_classify` and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_shield_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// Return the version string.
#[unsafe(no_mangle)]
pub extern "C" fn call_shield_version() -> *const c_char {
    c"0.1.0".as_ptr()
}

// s0=spam patterns
const SPAM: &[(&str, f64)] = &[
    ("extended warranty", 0.95),
    ("car warranty", 0.95),
    ("been trying to reach you", 0.90),
    ("courtesy call", 0.85),
    ("special offer", 0.85),
    ("selected for", 0.80),
    ("press 1", 0.90),
    ("press one", 0.90),
    ("limited time", 0.80),
    ("act now", 0.80),
    ("free gift", 0.85),
    ("congratulations you", 0.85),
    ("you have won", 0.90),
    ("lower your rate", 0.85),
    ("reduce your debt", 0.85),
    ("the irs", 0.80),
    ("irs agent", 0.85),
    ("social security number", 0.95),
    ("arrest warrant", 0.95),
    ("legal action", 0.80),
    ("final notice", 0.85),
    ("from your bank", 0.70),
    ("verify your account", 0.85),
    ("confirm your identity", 0.80),
];

// s1=legit patterns
const LEGIT: &[(&str, f64)] = &[
    ("appointment", 0.80),
    ("confirming your", 0.85),
    ("returning your call", 0.90),
    ("you called us", 0.85),
    ("this is dr", 0.80),
    ("this is doctor", 0.80),
    ("your order", 0.70),
    ("delivery", 0.70),
    ("picking up", 0.75),
    ("schedule", 0.70),
    ("follow up", 0.70),
    ("checking in", 0.65),
    ("your application", 0.65),
    ("interview", 0.80),
];

fn classify(text: &str) -> (&'static str, f64, String) {
    let mut spam_max = 0.0_f64;
    let mut legit_max = 0.0_f64;
    let mut matched = Vec::new();

    for (pattern, weight) in SPAM {
        if text.contains(pattern) {
            if *weight > spam_max { spam_max = *weight; }
            matched.push(*pattern);
        }
    }
    for (pattern, weight) in LEGIT {
        if text.contains(pattern) {
            if *weight > legit_max { legit_max = *weight; }
            matched.push(*pattern);
        }
    }

    let joined = matched.join(", ");
    if spam_max > legit_max && spam_max > 0.5 {
        ("SPAM", spam_max, joined)
    } else if legit_max > spam_max && legit_max > 0.5 {
        ("LEGITIMATE", legit_max, joined)
    } else {
        ("UNKNOWN", 0.5 - (spam_max - legit_max).abs(), joined)
    }
}

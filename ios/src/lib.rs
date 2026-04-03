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
    CString::new(result)
        .unwrap_or_else(|_| CString::new("UNKNOWN|0.50|").unwrap())
        .into_raw()
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

// s0/s1 — generated from patterns.csv at build time
include!(concat!(env!("OUT_DIR"), "/patterns.rs"));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spam_extended_warranty() {
        let (v, s, _) = classify("your extended warranty is expiring");
        assert_eq!(v, "SPAM");
        assert!(s >= 0.90);
    }

    #[test]
    fn legit_appointment() {
        let (v, s, _) = classify("confirming your appointment for tuesday");
        assert_eq!(v, "LEGITIMATE");
        assert!(s >= 0.80);
    }

    #[test]
    fn unknown_hello() {
        let (v, _, _) = classify("hello");
        assert_eq!(v, "UNKNOWN");
    }

    #[test]
    fn first_not_spam() {
        let (v, _, _) = classify("this is your first appointment");
        assert_ne!(v, "SPAM", "'first' must not match 'irs'");
    }

    #[test]
    fn birthday_not_spam() {
        let (v, _, _) = classify("happy birthday");
        assert_ne!(v, "SPAM");
    }

    #[test]
    fn from_your_bank_is_spam() {
        let (v, _, _) = classify("this is from your bank please confirm");
        assert_eq!(v, "SPAM");
    }

    #[test]
    fn verify_account_is_spam() {
        let (v, _, _) = classify("we need to verify your account");
        assert_eq!(v, "SPAM");
    }

    #[test]
    fn score_never_negative() {
        for input in ["", "hello", "random words"] {
            let (_, s, _) = classify(input);
            assert!(s >= 0.0, "score for '{input}' was {s}");
        }
    }
}

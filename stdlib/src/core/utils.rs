//! Zenith Standard Utils Module
//!
//! This module provides string manipulation and date/time utilities.

use chrono::{Local, Utc};

pub fn string_to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn string_to_lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn string_split(s: &str, sep: &str) -> Vec<String> {
    s.split(sep).map(|part| part.to_string()).collect()
}

pub fn string_contains(s: &str, sub: &str) -> bool {
    s.contains(sub)
}

pub fn string_replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn time_now_utc() -> String {
    Utc::now().to_rfc3339()
}

pub fn time_now_local() -> String {
    Local::now().to_rfc3339()
}

pub fn time_format(rfc3339_time: &str, format: &str) -> Result<String, String> {
    let dt: chrono::DateTime<chrono::FixedOffset> =
        chrono::DateTime::parse_from_rfc3339(rfc3339_time).map_err(|e| e.to_string())?;
    Ok(dt.format(format).to_string())
}

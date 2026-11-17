//! Field checks for manifest.json (split out of manifest.rs to keep each file short).

use super::manifest::{is_unsafe_char, is_valid_id, Manifest, MAX_COORD, MAX_EXTRACTS};
use std::collections::HashSet;

/// Visible text: bounded length and no control characters.
pub(super) fn check_text(field: &str, value: &str, max: usize, allow_empty: bool) -> Result<(), String> {
    if !allow_empty && value.trim().is_empty() {
        return Err(format!("\"{field}\" is empty"));
    }
    if value.chars().count() > max {
        return Err(format!("\"{field}\" is longer than {max} characters"));
    }
    if value.chars().any(is_unsafe_char) {
        return Err(format!("\"{field}\" contains control characters"));
    }
    Ok(())
}

pub(super) fn check_coord(field: &str, v: f64) -> Result<(), String> {
    if v.is_finite() && v.abs() <= MAX_COORD {
        Ok(())
    } else {
        Err(format!("\"{field}\" is not a valid number"))
    }
}

impl Manifest {
    pub(super) fn validate_extracts(&self) -> Result<(), String> {
        if self.extracts.len() > MAX_EXTRACTS {
            return Err(format!("at most {MAX_EXTRACTS} extracts"));
        }
        let mut seen = HashSet::new();
        for e in &self.extracts {
            if !is_valid_id(&e.id) || !seen.insert(e.id.as_str()) {
                return Err(format!("invalid or repeated extract id: {}", e.id));
            }
            check_text("extracts.name", &e.name, 60, false)?;
            check_text("extracts.conditions", &e.conditions, 300, true)?;
            if e.switches.len() > 8 {
                return Err("at most 8 switches per extract".into());
            }
            for sw in &e.switches {
                check_text("extracts.switches", sw, 60, false)?;
            }
            for v in [e.x, e.y, e.z] {
                check_coord("extracts.x/y/z", v)?;
            }
        }
        Ok(())
    }
}

//! Comprobación de imágenes sin decodificarlas: bytes mágicos, dimensiones y SVG seguro.

use super::manifest::MAX_IMAGE_SIDE;
use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Svg,
    Png,
    Jpeg,
    Webp,
}

impl Kind {
    /// Solo nombres planos, en minúsculas y con extensión permitida.
    pub fn from_file_name(name: &str) -> Option<Self> {
        let (stem, ext) = name.rsplit_once('.')?;
        let stem_ok = (1..=40).contains(&stem.len())
            && stem.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_');
        if !stem_ok {
            return None;
        }
        match ext {
            "svg" => Some(Self::Svg),
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "webp" => Some(Self::Webp),
            _ => None,
        }
    }

    pub fn mime(self) -> &'static str {
        match self {
            Self::Svg => "image/svg+xml",
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Webp => "image/webp",
        }
    }
}

fn be16(b: &[u8], at: usize) -> Option<u32> {
    let s = b.get(at..at + 2)?;
    Some(u32::from(u16::from_be_bytes([*s.first()?, *s.get(1)?])))
}

fn be32(b: &[u8], at: usize) -> Option<u32> {
    let s: [u8; 4] = b.get(at..at + 4)?.try_into().ok()?;
    Some(u32::from_be_bytes(s))
}

fn le(b: &[u8], at: usize, n: usize) -> Option<u32> {
    let s = b.get(at..at + n)?;
    Some(s.iter().rev().fold(0u32, |acc, x| (acc << 8) | u32::from(*x)))
}

fn png_size(b: &[u8]) -> Option<(u32, u32)> {
    let sig = b"\x89PNG\r\n\x1a\n";
    (b.get(..8)? == sig && b.get(12..16)? == b"IHDR").then_some(())?;
    Some((be32(b, 16)?, be32(b, 20)?))
}

fn jpeg_size(b: &[u8]) -> Option<(u32, u32)> {
    (b.get(..3)? == [0xFF, 0xD8, 0xFF]).then_some(())?;
    let mut i = 2;
    while i < b.len() {
        if *b.get(i)? != 0xFF {
            return None;
        }
        let marker = *b.get(i + 1)?;
        if marker == 0xFF {
            i += 1;
            continue;
        }
        if (0xD0..=0xD9).contains(&marker) || marker == 0x01 {
            i += 2;
            continue;
        }
        let len = be16(b, i + 2)? as usize;
        let is_sof = matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF);
        if is_sof {
            return Some((be16(b, i + 7)?, be16(b, i + 5)?));
        }
        i += 2 + len;
    }
    None
}

fn webp_size(b: &[u8]) -> Option<(u32, u32)> {
    (b.get(..4)? == b"RIFF" && b.get(8..12)? == b"WEBP").then_some(())?;
    match b.get(12..16)? {
        b"VP8 " => Some((le(b, 26, 2)? & 0x3FFF, le(b, 28, 2)? & 0x3FFF)),
        b"VP8L" => {
            (*b.get(20)? == 0x2F).then_some(())?;
            let bits = le(b, 21, 4)?;
            Some(((bits & 0x3FFF) + 1, ((bits >> 14) & 0x3FFF) + 1))
        }
        b"VP8X" => Some((le(b, 24, 3)? + 1, le(b, 27, 3)? + 1)),
        _ => None,
    }
}

/// Tipo y dimensiones de una imagen raster por sus bytes mágicos (sin decodificar).
pub fn sniff(bytes: &[u8]) -> Option<(Kind, (u32, u32))> {
    png_size(bytes)
        .map(|d| (Kind::Png, d))
        .or_else(|| webp_size(bytes).map(|d| (Kind::Webp, d)))
        .or_else(|| jpeg_size(bytes).map(|d| (Kind::Jpeg, d)))
}

/// Valida la imagen y devuelve sus dimensiones (en SVG, las del manifest).
pub fn check(kind: Kind, bytes: &[u8], declared: (u32, u32)) -> Result<(u32, u32), String> {
    let size = match kind {
        Kind::Svg => {
            check_svg(bytes)?;
            declared
        }
        Kind::Png => png_size(bytes).ok_or("the image is not a valid PNG")?,
        Kind::Jpeg => jpeg_size(bytes).ok_or("the image is not a valid JPEG")?,
        Kind::Webp => webp_size(bytes).ok_or("the image is not a valid WebP")?,
    };
    let side = 1..=MAX_IMAGE_SIDE;
    if !side.contains(&size.0) || !side.contains(&size.1) {
        return Err(format!("the image is {}x{}; the maximum is {MAX_IMAGE_SIDE}px per side", size.0, size.1));
    }
    if size != declared {
        return Err(format!("the image is {}x{} but the manifest says {}x{}", size.0, size.1, declared.0, declared.1));
    }
    Ok(size)
}

struct SvgRules {
    banned: Regex,
    refs: Regex,
    css_url: Regex,
}

fn svg_rules() -> Option<&'static SvgRules> {
    static R: OnceLock<Option<SvgRules>> = OnceLock::new();
    R.get_or_init(|| {
        Some(SvgRules {
            // Scripts, contenido HTML, DTD/entidades, manejadores on*, javascript: e @import.
            banned: Regex::new(
                r"(?i)<\s*script|<\s*foreignobject|<\s*iframe|<\s*embed|<\s*object|<!doctype|<!entity|<\?xml-stylesheet|\son[a-z]+\s*=|javascript:|@import",
            )
            .ok()?,
            refs: Regex::new(r#"(?i)(?:href|src)\s*=\s*["']\s*([^"']*)["']"#).ok()?,
            css_url: Regex::new(r#"(?i)url\(\s*["']?\s*([^"')\s]*)"#).ok()?,
        })
    })
    .as_ref()
}

/// Solo se admiten referencias internas (#id) e imágenes raster embebidas.
fn is_safe_ref(value: &str) -> bool {
    let v = value.trim().to_ascii_lowercase();
    v.starts_with('#') || ["data:image/png;", "data:image/jpeg;", "data:image/webp;"].iter().any(|p| v.starts_with(p))
}

pub fn check_svg(bytes: &[u8]) -> Result<(), String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "the SVG is not valid UTF-8")?;
    if !text.contains("<svg") {
        return Err("the file does not look like an SVG".into());
    }
    let rules = svg_rules().ok_or("could not prepare SVG validation")?;
    if let Some(m) = rules.banned.find(text) {
        return Err(format!("the SVG contains disallowed content ({})", m.as_str().trim()));
    }
    let refs = rules.refs.captures_iter(text).chain(rules.css_url.captures_iter(text));
    for cap in refs {
        let value = cap.get(1).map_or("", |m| m.as_str());
        if !is_safe_ref(value) {
            return Err("the SVG contains external references".into());
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "image_tests.rs"]
mod tests;

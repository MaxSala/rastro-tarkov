//! Del formato de json.tarkov.dev al catálogo compacto: nombres traducidos, ofertas en rublos,
//! mejor venta, valor por casilla, compra más barata y datos de munición.

use super::raw::{translate, ItemsDoc, Locale, RawItem, RawOffer, TradersDoc};
use super::{Ammo, Item, Offer};
use crate::maps::manifest::is_unsafe_char;
use std::collections::HashMap;

const MAX_ITEMS: usize = 20_000;

/// Calibres que tarkov.dev no muestra en su tabla (lanzagranadas, cañones…).
const SKIP_CALIBERS: &[&str] =
    &["Caliber30x29", "Caliber40x46", "Caliber127x108", "Caliber26x75", "Caliber40mmRU", "Caliber20x1mm", "Caliber725"];

/// Nombre visible de cada calibre interno.
const CALIBERS: &[(&str, &str)] = &[
    ("Caliber366TKM", ".366 TKM"),
    ("Caliber556x45NATO", "5.56x45 mm"),
    ("Caliber1143x23ACP", ".45 ACP"),
    ("Caliber127x55", "12.7x55 mm"),
    ("Caliber127x99", ".50 BMG"),
    ("Caliber23x75", "23x75 mm"),
    ("Caliber46x30", "4.6x30 mm"),
    ("Caliber545x39", "5.45x39 mm"),
    ("Caliber57x28", "5.7x28 mm"),
    ("Caliber762x25TT", "7.62x25 mm TT"),
    ("Caliber762x35", ".300 Blackout"),
    ("Caliber762x39", "7.62x39 mm"),
    ("Caliber762x51", "7.62x51 mm"),
    ("Caliber762x54R", "7.62x54R"),
    ("Caliber86x70", ".338 Lapua Magnum"),
    ("Caliber9x18PM", "9x18 mm"),
    ("Caliber9x19PARA", "9x19 mm"),
    ("Caliber9x21", "9x21 mm"),
    ("Caliber9x39", "9x39 mm"),
    ("Caliber20g", "20 ga"),
    ("Caliber12g", "12 ga"),
    ("Caliber9x33R", ".357 Magnum"),
    ("Caliber68x51", "6.8x51 mm"),
    ("Caliber127x33", ".50 AE"),
    ("Caliber93x64", "9.3x64 mm"),
    ("Caliber784x49", ".308 ME"),
];

fn caliber_name(raw: &str, ammo_type: Option<&str>) -> String {
    let base = CALIBERS
        .iter()
        .find(|(k, _)| *k == raw)
        .map_or_else(|| raw.trim_start_matches("Caliber").to_owned(), |(_, v)| (*v).to_owned());
    if base.ends_with(" ga") {
        // Escopetas: postas y balas son mundos distintos; se separan como en tarkov.dev.
        let kind = if ammo_type == Some("bullet") { "slug" } else { "shot" };
        return format!("{base} {kind}");
    }
    base
}

/// normalizedName para el enlace a tarkov.dev/item/…: minúsculas, dígitos y guiones.
fn valid_slug(s: &str) -> bool {
    (1..=80).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn text(raw: &str, max: usize) -> String {
    raw.chars().filter(|c| !is_unsafe_char(*c)).take(max).collect::<String>().trim().to_owned()
}

fn rub(v: Option<f64>) -> Option<u32> {
    v.filter(|p| p.is_finite() && *p > 0.0 && *p < 4.0e9).map(|p| p.round() as u32)
}

fn finite(v: Option<f64>, max: f64) -> Option<f64> {
    v.filter(|x| x.is_finite() && x.abs() <= max)
}

fn offer(o: &RawOffer, traders: &HashMap<String, (String, String)>) -> Option<Offer> {
    let id = o.trader.as_deref()?;
    let (slug, name) = traders.get(id)?;
    let price_rub = rub(o.price_rub)?;
    let currency = o.currency.as_deref().unwrap_or("RUB");
    let original = (currency != "RUB").then(|| finite(o.price, 1.0e9).map(|p| (p, text(currency, 4)))).flatten();
    let level = o.min_trader_level.filter(|l| (1.0..=10.0).contains(l)).map(|l| l as u8);
    Some(Offer { vendor: slug.clone(), vendor_name: name.clone(), price_rub, original, level })
}

fn ammo(raw: &RawItem) -> Option<Ammo> {
    let p = raw.properties.as_ref()?;
    let cal = p.caliber.as_deref()?;
    if !raw.types.iter().any(|t| t == "ammo") || SKIP_CALIBERS.contains(&cal) {
        return None;
    }
    Some(Ammo {
        caliber: caliber_name(cal, p.ammo_type.as_deref()),
        damage: finite(p.damage, 10_000.0)?,
        projectiles: finite(p.projectile_count, 100.0).map_or(1, |n| n.max(1.0) as u32),
        penetration: finite(p.penetration_power, 1000.0).unwrap_or(0.0),
        armor_damage: finite(p.armor_damage, 1000.0).unwrap_or(0.0),
        fragmentation: finite(p.fragmentation_chance, 1.0).map_or(0.0, |f| f * 100.0),
        speed: finite(p.initial_speed, 10_000.0).unwrap_or(0.0),
        tracer: p.tracer.unwrap_or(false),
    })
}

pub struct Locales {
    pub primary: Locale,
    pub fallback: Locale,
}

/// Comerciantes: id → (normalizedName, nombre traducido).
pub fn traders(doc: &TradersDoc, l: &Locales) -> HashMap<String, (String, String)> {
    doc.data
        .iter()
        .filter_map(|(id, t)| {
            let slug = t.normalized_name.as_deref().filter(|s| crate::maps::manifest::is_valid_id(s))?;
            let name = text(&translate(t.name.as_deref().unwrap_or(slug), &l.primary, &l.fallback), 40);
            Some((id.clone(), (slug.to_owned(), name)))
        })
        .collect()
}

pub fn items(doc: ItemsDoc, traders: &HashMap<String, (String, String)>, l: &Locales, flea_name: &str) -> Vec<Item> {
    let mut out: Vec<Item> = doc
        .data
        .items
        .into_values()
        .filter_map(|raw| {
            let valid_id = raw.id.len() == 24 && raw.id.bytes().all(|b| b.is_ascii_hexdigit());
            if !valid_id {
                return None;
            }
            let key = raw.name.as_deref()?;
            let name = text(&translate(key, &l.primary, &l.fallback), 120);
            // Sin traducción en ningún idioma solo queda la clave interna («<id> Name»): fuera.
            if name == key && key.starts_with(&raw.id) {
                return None;
            }
            let short = text(&translate(raw.short_name.as_deref().unwrap_or(""), &l.primary, &l.fallback), 40);
            let no_flea = raw.types.iter().any(|t| t == "noFlea");
            let flea_low = rub(raw.last_low_price).filter(|_| !no_flea);
            let flea_avg = rub(raw.avg24h_price).filter(|_| !no_flea);
            let flea = |price: u32| Offer {
                vendor: "flea".into(),
                vendor_name: flea_name.into(),
                price_rub: price,
                original: None,
                level: None,
            };
            let mut sell: Vec<Offer> = raw.sell_to_trader.iter().flatten().filter_map(|o| offer(o, traders)).collect();
            sell.extend(flea_low.map(flea));
            sell.sort_by_key(|o| std::cmp::Reverse(o.price_rub));
            let mut buy: Vec<Offer> = raw.buy_from_trader.iter().flatten().filter_map(|o| offer(o, traders)).collect();
            buy.extend(flea_avg.or(flea_low).map(flea));
            buy.sort_by_key(|o| o.price_rub);
            let (w, h) = (
                raw.width.map_or(1.0, |v| v.clamp(1.0, 20.0)) as u8,
                raw.height.map_or(1.0, |v| v.clamp(1.0, 20.0)) as u8,
            );
            let best_sell = sell.first().cloned();
            let per_slot = best_sell.as_ref().map(|o| o.price_rub / u32::from(w) / u32::from(h));
            let types = raw
                .types
                .iter()
                .filter(|t| t.len() <= 20 && t.bytes().all(|b| b.is_ascii_alphanumeric()))
                .take(12)
                .cloned()
                .collect();
            Some(Item {
                ammo: ammo(&raw),
                id: raw.id.clone(),
                slug: raw.normalized_name.as_deref().filter(|s| valid_slug(s)).unwrap_or("").to_owned(),
                name,
                short,
                width: w,
                height: h,
                types,
                flea_low,
                flea_avg,
                change48h: finite(raw.change_last48h_percent, 100_000.0),
                cheapest_buy: buy.first().cloned(),
                sell,
                buy,
                best_sell,
                per_slot,
            })
        })
        .filter(|i| !i.name.is_empty())
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out.truncate(MAX_ITEMS);
    out
}

#[cfg(test)]
#[path = "build_tests.rs"]
mod tests;

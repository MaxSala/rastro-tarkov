//! Formato de json.tarkov.dev (`{modo}/items`, `{modo}/traders` y sus traducciones `_{idioma}`).
//! Referencia: el propio cliente de tarkov.dev (src/features/items/do-fetch-items.mjs y
//! src/modules/api-request.mjs). Todo es opcional: lo que falte o no encaje se ignora.

use serde::de::IgnoredAny;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ItemsDoc {
    pub data: ItemsData,
}

#[derive(Deserialize)]
pub struct ItemsData {
    #[serde(default)]
    pub items: HashMap<String, RawItem>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawItem {
    pub id: String,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub normalized_name: Option<String>,
    pub types: Vec<String>,
    pub avg24h_price: Option<f64>,
    pub last_low_price: Option<f64>,
    pub change_last48h_percent: Option<f64>,
    pub base_price: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub buy_from_trader: Option<Vec<RawOffer>>,
    pub sell_to_trader: Option<Vec<RawOffer>>,
    pub properties: Option<RawProps>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawOffer {
    pub trader: Option<String>,
    pub price: Option<f64>,
    pub currency: Option<String>,
    #[serde(rename = "priceRUB")]
    pub price_rub: Option<f64>,
    pub min_trader_level: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawProps {
    pub caliber: Option<String>,
    pub ammo_type: Option<String>,
    pub damage: Option<f64>,
    pub projectile_count: Option<f64>,
    pub penetration_power: Option<f64>,
    pub armor_damage: Option<f64>,
    pub fragmentation_chance: Option<f64>,
    pub initial_speed: Option<f64>,
    pub tracer: Option<bool>,
}

#[derive(Deserialize)]
pub struct TradersDoc {
    #[serde(default)]
    pub data: HashMap<String, RawTrader>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawTrader {
    pub name: Option<String>,
    pub normalized_name: Option<String>,
}

/// Traducciones: clave → texto. Valores que no sean texto se ignoran.
#[derive(Deserialize)]
#[serde(untagged)]
enum Text {
    S(String),
    Other(IgnoredAny),
}

#[derive(Deserialize)]
struct LocaleDoc {
    #[serde(default)]
    data: HashMap<String, Text>,
}

pub type Locale = HashMap<String, String>;

pub fn parse_locale(bytes: &[u8]) -> Locale {
    serde_json::from_slice::<LocaleDoc>(bytes)
        .map(|d| d.data.into_iter().filter_map(|(k, v)| if let Text::S(s) = v { Some((k, s)) } else { None }).collect())
        .unwrap_or_default()
}

/// Igual que tarkov.dev: idioma principal, luego inglés, luego la propia clave.
pub fn translate(key: &str, primary: &Locale, fallback: &Locale) -> String {
    primary.get(key).or_else(|| fallback.get(key)).cloned().unwrap_or_else(|| key.to_owned())
}

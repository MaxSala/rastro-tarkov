//! Mercado: precios de objetos y munición de tarkov.dev, descargados solo cuando el usuario lo pide.
//! Un único catálogo compacto sirve al buscador de precios y a la tabla de munición.

pub mod build;
pub mod fetch;
pub mod icons;
pub mod raw;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    /// `flea` o el normalizedName del comerciante.
    pub vendor: String,
    pub vendor_name: String,
    pub price_rub: u32,
    /// Precio en la moneda original si no es rublo (p. ej. 145 USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<(f64, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ammo {
    pub caliber: String,
    pub damage: f64,
    pub projectiles: u32,
    pub penetration: f64,
    /// Porcentajes ya multiplicados por 100.
    pub armor_damage: f64,
    pub fragmentation: f64,
    pub speed: f64,
    pub tracer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub name: String,
    pub short: String,
    pub slug: String,
    pub width: u8,
    pub height: u8,
    pub types: Vec<String>,
    pub flea_low: Option<u32>,
    pub flea_avg: Option<u32>,
    pub change48h: Option<f64>,
    pub sell: Vec<Offer>,
    pub buy: Vec<Offer>,
    /// Mejor venta (comerciante o mercadillo) y valor por casilla.
    pub best_sell: Option<Offer>,
    pub per_slot: Option<u32>,
    pub cheapest_buy: Option<Offer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo: Option<Ammo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub mode: String,
    pub lang: String,
    /// Segundos Unix de la descarga.
    pub fetched_at: u64,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MarketState {
    #[default]
    Empty,
    Loading,
    Ready,
    Error,
}

/// Estado visible en la instantánea (el catálogo en sí va por HTTP: es grande).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketStatus {
    pub state: MarketState,
    pub fetched_at: Option<u64>,
    pub count: usize,
    pub mode: Option<String>,
    pub lang: Option<String>,
    pub error: Option<String>,
}

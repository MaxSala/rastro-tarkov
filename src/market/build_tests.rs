use super::*;
use crate::market::raw::{parse_locale, ItemsDoc, TradersDoc};

// Estructura de json.tarkov.dev según el cliente oficial (do-fetch-items.mjs): `data.items` es un
// objeto por id, los nombres son claves de traducción y las ofertas apuntan al id del comerciante.
const ITEMS: &str = r#"{"data":{"items":{
 "5c0d5e4486f77478390952fe":{"id":"5c0d5e4486f77478390952fe","name":"5c0d5e4486f77478390952fe Name","shortName":"5c0d5e4486f77478390952fe ShortName",
   "normalizedName":"545x39mm-pp-gs","types":["ammo"],"width":1,"height":1,"avg24hPrice":420,"lastLowPrice":399.4,"changeLast48hPercent":-3.2,
   "buyFromTrader":[{"trader":"54cb50c76803fa8b248b4571","price":2.1,"currency":"USD","priceRUB":291.6,"minTraderLevel":2}],
   "sellToTrader":[{"trader":"54cb50c76803fa8b248b4571","price":40,"currency":"RUB","priceRUB":40}],
   "properties":{"caliber":"Caliber545x39","ammoType":"bullet","damage":48,"penetrationPower":40,"armorDamage":46,"fragmentationChance":0.17,"initialSpeed":890,"tracer":false}},
 "59faff1d86f7746c51718c9c":{"id":"59faff1d86f7746c51718c9c","name":"59faff1d86f7746c51718c9c Name","shortName":"59faff1d86f7746c51718c9c ShortName",
   "normalizedName":"physical-bitcoin","types":["barter"],"width":1,"height":1,"avg24hPrice":null,"lastLowPrice":null,
   "sellToTrader":[{"trader":"54cb50c76803fa8b248b4571","price":250000,"currency":"RUB","priceRUB":250000},{"trader":"desconocido","priceRUB":1}]},
 "5c94bbff86f7747ee735c08f":{"id":"5c94bbff86f7747ee735c08f","name":"5c94bbff86f7747ee735c08f Name","types":["keys","noFlea"],"width":1,"height":1,
   "avg24hPrice":90000,"lastLowPrice":80000,"sellToTrader":[]},
 "no-es-un-id":{"id":"../x","name":"x"},
 "5a7c4850e899ef00150be885":{"id":"5a7c4850e899ef00150be885","name":"5a7c4850e899ef00150be885 Name","types":["ammo"],"width":1,"height":1,
   "properties":{"caliber":"Caliber40x46","damage":199}},
 "5c0d688c86f77413ae3407b2":{"id":"5c0d688c86f77413ae3407b2","name":"5c0d688c86f77413ae3407b2 Name","types":["barter"]},
 "5d6e6772a4b936088465b17c":{"id":"5d6e6772a4b936088465b17c","name":"5d6e6772a4b936088465b17c Name","types":["ammo"],"width":1,"height":1,
   "properties":{"caliber":"Caliber12g","ammoType":"buckshot","damage":37,"projectileCount":8,"penetrationPower":3,"fragmentationChance":0}}
}}}"#;
const TRADERS: &str =
    r#"{"data":{"54cb50c76803fa8b248b4571":{"name":"54cb50c76803fa8b248b4571 Nickname","normalizedName":"prapor"}}}"#;
const ES: &str = r#"{"data":{"5c0d5e4486f77478390952fe Name":"5.45x39 mm PP gs","5c0d5e4486f77478390952fe ShortName":"PP",
  "59faff1d86f7746c51718c9c Name":"Bitcoin físico","54cb50c76803fa8b248b4571 Nickname":"Prapor\u202e","otra":3}}"#;
const EN: &str = r#"{"data":{"5c94bbff86f7747ee735c08f Name":"TerraGroup Labs access keycard","5a7c4850e899ef00150be885 Name":"40x46 mm M381 (HE)","5d6e6772a4b936088465b17c Name":"12/70 7mm buckshot"}}"#;

fn catalog() -> Vec<Item> {
    let doc: ItemsDoc = serde_json::from_str(ITEMS).expect("items");
    let tdoc: TradersDoc = serde_json::from_str(TRADERS).expect("traders");
    let l = Locales { primary: parse_locale(ES.as_bytes()), fallback: parse_locale(EN.as_bytes()) };
    let t = traders(&tdoc, &l);
    items(doc, &t, &l, "Mercadillo")
}

#[test]
fn builds_prices_offers_and_translations() {
    let items = catalog();
    let names: Vec<&String> = items.iter().map(|i| &i.name).collect();
    assert_eq!(items.len(), 5, "fuera el id no válido y el objeto sin traducción: {names:?}");
    assert!(!names.iter().any(|n| n.ends_with(" Name")));
    let grenade = items.iter().find(|i| i.name.starts_with("40x46")).expect("granada");
    assert!(grenade.ammo.is_none(), "lanzagranadas: en precios sí, en la tabla de munición no");
    let pp = items.iter().find(|i| i.short == "PP").expect("PP");
    assert_eq!(pp.name, "5.45x39 mm PP gs");
    assert_eq!((pp.flea_low, pp.flea_avg), (Some(399), Some(420)));
    let best = pp.best_sell.as_ref().expect("venta");
    assert_eq!((best.vendor.as_str(), best.price_rub), ("flea", 399));
    let buy = pp.cheapest_buy.as_ref().expect("compra");
    assert_eq!(
        (buy.vendor.as_str(), buy.vendor_name.as_str(), buy.price_rub, buy.level),
        ("prapor", "Prapor", 292, Some(2))
    );
    assert_eq!(buy.original, Some((2.1, "USD".into())));
    let ammo = pp.ammo.as_ref().expect("munición");
    assert_eq!((ammo.caliber.as_str(), ammo.penetration, ammo.fragmentation.round()), ("5.45x39 mm", 40.0, 17.0));
}

#[test]
fn no_flea_items_and_fallbacks() {
    let items = catalog();
    let card = items.iter().find(|i| i.name == "TerraGroup Labs access keycard").expect("respaldo en inglés");
    assert_eq!(card.flea_low, None, "noFlea no tiene precio de mercadillo");
    assert!(card.best_sell.is_none());
    let btc = items.iter().find(|i| i.name == "Bitcoin físico").expect("btc");
    assert_eq!(btc.sell.len(), 1, "oferta de comerciante desconocido descartada");
    assert_eq!(btc.per_slot, Some(250_000));
    let shot = items.iter().find(|i| i.name == "12/70 7mm buckshot").expect("postas");
    let a = shot.ammo.as_ref().expect("munición");
    assert_eq!((a.caliber.as_str(), a.projectiles), ("12 ga shot", 8));
}

#[test]
fn garbage_is_rejected() {
    assert!(serde_json::from_str::<ItemsDoc>("[]").is_err());
    assert!(parse_locale(b"no json").is_empty());
}

//! Instalación automática de mapas desde tarkov.dev (solo cuando el usuario lo pide).
//!
//! Fuentes fijas: `raw.githubusercontent.com` (maps.json de tarkov.dev y los SVG del proyecto
//! tarkov-dev-svg-maps), `json.tarkov.dev` (datos de mapa, fuente principal) y `api.tarkov.dev`
//! (GraphQL, respaldo).
//! Todo pasa por la misma validación que un paquete importado a mano.

pub mod api;
pub mod calib;
pub mod icons;
pub mod jsondoc;
pub mod locks;
pub mod sync;

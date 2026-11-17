//! Script de compilación:
//! - Incrusta el icono y la versión en el .exe de Windows (embed-resource; en otros sistemas no hace nada).
//! - Recompila la app cuando cambia la web compilada (web/dist), para que nunca quede una interfaz vieja.

fn main() {
    println!("cargo:rerun-if-changed=rastro.rc");
    println!("cargo:rerun-if-changed=assets/brand/rastro.ico");
    println!("cargo:rerun-if-changed=web/dist");
    println!("cargo:rerun-if-changed=web/dist/index.html");
    // Sin compilador de recursos (raro con Visual Studio Build Tools) el .exe sale igual, sin icono.
    if let Err(e) = embed_resource::compile("rastro.rc", embed_resource::NONE).manifest_optional() {
        println!("cargo:warning=No se pudo incrustar el icono en el .exe: {e}");
    }
}

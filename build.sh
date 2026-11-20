#!/usr/bin/env sh
# Compila Rastro completo: primero la interfaz (web/dist), luego el binario que la incrusta.
set -eu
cd "$(dirname "$0")"
echo "==> Interfaz (web)"
# Solo lo que va dentro de la app: falla si tiene vulnerabilidades conocidas.
( cd web && npm ci && npm audit --omit=dev --audit-level=moderate && npm run build )
echo "==> Aplicación (Rust, release)"
cargo build --release
echo "==> Listo: target/release/rastro"

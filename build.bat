@echo off
rem Compila Rastro completo: primero la interfaz (web\dist), luego rastro.exe que la incrusta.
setlocal
cd /d "%~dp0"
echo ==^> Interfaz (web)
pushd web
call npm ci || goto :error
rem Solo lo que va dentro de la app: falla si tiene vulnerabilidades conocidas.
call npm audit --omit=dev --audit-level=moderate || goto :error
call npm run build || goto :error
popd
echo ==^> Aplicacion (Rust, release)
cargo build --release || goto :error
echo ==^> Listo: target\release\rastro.exe
exit /b 0
:error
echo.
echo La compilacion ha fallado. Revisa el mensaje de arriba.
exit /b 1

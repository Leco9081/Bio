@echo off
echo === Compilando Bio para Windows ===
where cargo >nul 2>nul
if errorlevel 1 goto semrust
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release
if errorlevel 1 goto erro
if exist Bio_Windows rmdir /s /q Bio_Windows
mkdir Bio_Windows
mkdir Bio_Windows\skins
copy /y target\release\bio.exe Bio_Windows\bio.exe
copy /y TUTORIAL.txt Bio_Windows\TUTORIAL.txt
echo.
echo Pronto! A pasta Bio_Windows esta pronta para virar .zip
pause
exit /b 0
:semrust
echo Rust nao encontrado. Instale em https://rustup.rs e rode de novo.
pause
exit /b 1
:erro
echo ERRO na compilacao. Copie o texto do erro e mande para o Claude.
pause
exit /b 1

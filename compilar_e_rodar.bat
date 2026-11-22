@echo off
setlocal EnableExtensions
title Bio - compilar e rodar
cd /d "%~dp0"

echo ==================================================
echo   BIO 1.5 - criar pastas, compilar e abrir o jogo
echo ==================================================
echo.

rem ---------- 1) pastas ----------
if not exist src mkdir src
if not exist Bio_Windows mkdir Bio_Windows
if not exist Bio_Windows\skins mkdir Bio_Windows\skins
if not exist src\main.rs if exist main.rs copy /y main.rs src\main.rs >nul
if not exist Cargo.toml goto sem_projeto

rem ---------- 2) o Rust esta instalado? ----------
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
where cargo >nul 2>nul
if not errorlevel 1 goto compilar

rem sem Rust, mas ja existe um bio.exe pronto (caso do Winlator): so abre o jogo
if exist "Bio_Windows\bio.exe" (
  echo Rust nao encontrado, mas ja existe Bio_Windows\bio.exe. Abrindo o jogo...
  goto rodar
)

rem sem Rust e sem bio.exe: no Winlator/Wine nao da para instalar sozinho
reg query "HKCU\Software\Wine" >nul 2>nul
if not errorlevel 1 goto sem_rust_wine

goto instalar_rust

:instalar_rust
echo O Rust (compilador) nao esta instalado neste computador.
set "RESP="
set /p RESP=Baixar e instalar o Rust agora (precisa de internet)? (S/N): 
if /i "%RESP%"=="S" goto baixar_rust
if /i "%RESP%"=="Y" goto baixar_rust
goto sem_rust

:baixar_rust
echo Baixando rustup-init.exe ...
powershell -NoProfile -ExecutionPolicy Bypass -Command "[Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -UseBasicParsing -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe"
if not exist rustup-init.exe goto erro_download
echo Instalando o Rust (alguns minutos)...
rustup-init.exe -y --profile minimal
if errorlevel 1 goto erro_rustup
del rustup-init.exe >nul 2>nul
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
goto compilar

:compilar
echo.
echo Compilando o jogo (a primeira vez demora alguns minutos)...
set "RUSTFLAGS=-C target-feature=+crt-static"
cargo build --release
if errorlevel 1 goto erro_compilar
copy /y "target\release\bio.exe" "Bio_Windows\bio.exe" >nul
if errorlevel 1 goto erro_copiar
if exist TUTORIAL.txt copy /y TUTORIAL.txt "Bio_Windows\TUTORIAL.txt" >nul
echo Compilado com sucesso.
goto rodar

:rodar
echo.
echo Abrindo o jogo...
pushd Bio_Windows
start "Bio" "bio.exe"
popd
echo Pronto! O jogo abriu em outra janela. Pode fechar esta.
exit /b 0

:sem_projeto
echo Nao achei o Cargo.toml nesta pasta.
echo Extraia o zip inteiro e rode este arquivo de dentro da pasta extraida.
goto fim_erro

:sem_rust_wine
echo Detectei o Winlator/Wine e nao existe Bio_Windows\bio.exe ainda.
echo Compilar dentro do Winlator nao e recomendado (muito lento e pesado).
echo Gere o bio.exe antes, no PC ou pelo GitHub (veja o TUTORIAL.txt),
echo coloque-o na pasta Bio_Windows e rode este arquivo de novo.
goto fim_erro

:sem_rust
echo Sem o Rust nao da para compilar. Instale em https://rustup.rs
echo ou gere o bio.exe pelo GitHub (veja o TUTORIAL.txt).
goto fim_erro

:erro_download
echo ERRO: nao consegui baixar o rustup-init.exe (internet ou PowerShell).
echo Instale o Rust manualmente em https://rustup.rs e rode este arquivo de novo.
goto fim_erro

:erro_rustup
echo ERRO ao instalar o Rust. Instale manualmente em https://rustup.rs
goto fim_erro

:erro_copiar
echo ERRO: a compilacao terminou mas nao achei target\release\bio.exe
goto fim_erro

:erro_compilar
echo.
echo ERRO na compilacao. Copie o texto do erro acima e mande para o Claude.
if defined TENTOU_BUILDTOOLS goto fim_erro
where winget >nul 2>nul
if errorlevel 1 goto fim_erro
echo.
echo Se o erro fala de "link.exe" ou "MSVC", falta o Build Tools do Visual Studio (C++).
set "RESP2="
set /p RESP2=Tentar instalar o Build Tools agora com o winget? (S/N): 
if /i "%RESP2%"=="S" goto instalar_buildtools
if /i "%RESP2%"=="Y" goto instalar_buildtools
goto fim_erro

:instalar_buildtools
set "TENTOU_BUILDTOOLS=1"
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--passive --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
goto compilar

:fim_erro
echo.
pause
exit /b 1

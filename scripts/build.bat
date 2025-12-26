@echo off
REM Jagannath Build Script for Windows

echo Building Jagannath...

set MODE=%1
if "%MODE%"=="" set MODE=release

if "%MODE%"=="debug" goto debug
if "%MODE%"=="sattva" goto debug
if "%MODE%"=="release" goto release
if "%MODE%"=="rajas" goto release
if "%MODE%"=="minimal" goto minimal
if "%MODE%"=="tamas" goto minimal

echo Unknown mode: %MODE%
echo Usage: build.bat [debug^|release^|minimal]
exit /b 1

:debug
echo Building in Sattva (debug) mode...
cargo build
goto done

:release
echo Building in Rajas (release) mode...
cargo build --release
goto done

:minimal
echo Building in Tamas (minimal) mode...
cargo build --release --features minimal
goto done

:done
echo Build complete!

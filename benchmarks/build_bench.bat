@echo off
REM Jagannath Compiler Benchmark Build Script
REM Builds both C and Jagannath fibonacci benchmarks

echo === Jagannath Compiler Benchmark Build ===
echo.

REM Check for required tools
where gcc >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ERROR: gcc not found. Please install MinGW-w64 or MSYS2.
    exit /b 1
)

REM Create output directory
if not exist "benchmarks\bin" mkdir "benchmarks\bin"

REM === Build C version ===
echo [1/3] Building C fibonacci benchmark...
gcc -O3 -march=native -o benchmarks\bin\fibonacci_c.exe benchmarks\vs_c\compute\fibonacci.c
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to compile C benchmark
    exit /b 1
)
echo      OK: benchmarks\bin\fibonacci_c.exe

REM === Compile Jagannath version ===
echo [2/3] Compiling Jagannath fibonacci to assembly...
cargo run --bin jagc --release -- examples\fibonacci.jag --emit-asm -o benchmarks\bin\fibonacci_jag.s
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to compile Jagannath benchmark
    exit /b 1
)
echo      OK: benchmarks\bin\fibonacci_jag.s

REM === Assemble Jagannath (requires working assembly) ===
echo [3/3] Assembling Jagannath (requires runtime linkage)...
echo      NOTE: Full binary generation requires assembly of .s file
echo      For now, compare compilation times only.

echo.
echo === Build Complete ===
echo.
echo To run C benchmark:
echo   benchmarks\bin\fibonacci_c.exe [N]
echo.

@echo off
REM Run all tests

echo Running Jagannath tests...

REM Unit tests
echo Running unit tests...
cargo test --workspace
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

REM Integration tests
echo Running integration tests...
cargo test --workspace --test *
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo All tests passed!

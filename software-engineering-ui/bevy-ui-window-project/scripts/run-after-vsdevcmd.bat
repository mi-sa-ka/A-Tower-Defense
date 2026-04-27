@echo off
setlocal

set "PROJECT_DIR=%~dp0.."
set "VSDEVCMD="

if exist "%ProgramFiles%\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles%\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles%\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles%\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles(x86)%\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat" set "VSDEVCMD=%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat"
if not "%VSDEVCMD%"=="" goto found_vsdevcmd

echo VsDevCmd.bat not found. Please install Visual Studio Build Tools with C++ workload.
exit /b 1

:found_vsdevcmd
call "%VSDEVCMD%" -no_logo -arch=x64 -host_arch=x64

if exist "%USERPROFILE%\.cargo\bin" set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

echo Using VsDevCmd: %VSDEVCMD%
where link >nul 2>nul
if errorlevel 1 (
  echo link.exe is not available in PATH after VsDevCmd.
  echo Install C++ Build Tools workload by running this in Administrator PowerShell:
  echo winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --source winget --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"
  exit /b 1
)
where cargo >nul 2>nul
if errorlevel 1 (
  echo cargo is not available in PATH. Make sure Rust is installed for this user.
  exit /b 1
)

cd /d "%PROJECT_DIR%"
cargo check
if errorlevel 1 exit /b 1

cargo run

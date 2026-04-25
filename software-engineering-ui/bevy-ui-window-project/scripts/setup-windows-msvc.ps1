param(
    [switch]$TryInstallBuildTools,
    [switch]$RunCheck
)

$ErrorActionPreference = 'Stop'

function Write-Step($msg) {
    Write-Host "`n==> $msg" -ForegroundColor Cyan
}

function Check-Command($name) {
    return [bool](Get-Command $name -ErrorAction SilentlyContinue)
}

Write-Step "Check Rust and MSVC environment"
$cargoOk = Check-Command "cargo"
$rustupOk = Check-Command "rustup"
$linkOk = Check-Command "link.exe"

Write-Host "cargo:    $cargoOk"
Write-Host "rustup:   $rustupOk"
Write-Host "link.exe: $linkOk"

if (-not $cargoOk) {
    Write-Host "`ncargo was not found. Install Rust first: https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
}

if (-not $linkOk) {
    Write-Host "`nlink.exe (MSVC linker) was not found." -ForegroundColor Yellow
    Write-Host "Visual Studio Build Tools with C++ workload is required."

    if ($TryInstallBuildTools) {
        $wingetOk = Check-Command "winget"
        if (-not $wingetOk) {
            Write-Host "winget is not available. Install Build Tools manually:" -ForegroundColor Yellow
            Write-Host "https://visualstudio.microsoft.com/visual-cpp-build-tools/"
        }
        else {
            Write-Step "Install Visual Studio 2022 Build Tools using winget"
            winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --source winget --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"
            Write-Host "Install command executed. Restart terminal after installation." -ForegroundColor Green
        }
    }
    else {
        Write-Host "Run this command in Administrator PowerShell to install automatically:"
        Write-Host 'winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --source winget --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"'
    }
}

if ($RunCheck) {
    if (-not $cargoOk) {
        Write-Host "`nSkip cargo check: cargo is not installed." -ForegroundColor Yellow
        exit 1
    }

    Write-Step "Run cargo check in project directory"
    Push-Location (Join-Path $PSScriptRoot "..")
    try {
        cargo check
    }
    finally {
        Pop-Location
    }
}

Write-Host "`nDone." -ForegroundColor Green

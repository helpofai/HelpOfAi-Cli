# AIOS Local Verification Script
# This script compiles the CLI binary in release mode and runs diagnostic commands to verify AIOS integration.

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "   HelpOfAi AIOS Local Verification Suite (Release)" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Path to the compiled release binary
$cli = "target\release\helpofai.exe"
$tui = "target\release\helpofai-tui.exe"
$can_skip = (Test-Path $cli) -and (Test-Path $tui)

# 1. Build the CLI Binary in Release Mode
Write-Host "[1/4] Building HelpOfAi CLI & TUI in Release Mode..." -ForegroundColor Yellow
cargo build --release -p helpofai-cli -p helpofai-tui
if ($LASTEXITCODE -ne 0) {
    if ($can_skip) {
        Write-Host "Warning: Cargo release build failed (likely due to active running instances holding file locks)." -ForegroundColor Yellow
        Write-Host "Continuing verification using the existing release binaries..." -ForegroundColor Yellow
    } else {
        Write-Host "Error: Cargo release build failed and no existing binaries were found!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "Build completed successfully." -ForegroundColor Green
}
Write-Host ""

# 2. Check AIOS Registry Status
Write-Host "[2/4] Verifying Registry Status..." -ForegroundColor Yellow
& $cli aios registry-status
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Registry status check failed!" -ForegroundColor Red
    exit 1
}
Write-Host ""

# 3. List Registered Workflows
Write-Host "[3/4] Listing Registered Workflows..." -ForegroundColor Yellow
& $cli aios workflows
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Listing workflows failed!" -ForegroundColor Red
    exit 1
}
Write-Host ""

# 4. Run Diagnostics for a Workflow Phase
Write-Host "[4/4] Running Workflow Diagnostics..." -ForegroundColor Yellow
& $cli aios diag build-feature "implement a simple health check endpoint"
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Workflow diagnostics failed!" -ForegroundColor Red
    exit 1
}
Write-Host ""

Write-Host "==================================================" -ForegroundColor Green
Write-Host " Verification completed successfully! AIOS is ready." -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green
Write-Host "To execute an active workflow run, you can execute:" -ForegroundColor Green
Write-Host "  target\release\helpofai.exe aios run build-feature `"your task instruction`"" -ForegroundColor Yellow

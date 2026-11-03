$CargoArgs=$args
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$localCargo=Join-Path $root '.tools\cargo\bin'
if (Test-Path (Join-Path $localCargo 'cargo.exe')) {
    $env:CARGO_HOME=Join-Path $root '.tools\cargo'
    $env:RUSTUP_HOME=Join-Path $root '.tools\rustup'
    $env:PATH="$localCargo;$env:PATH"
    $toolBin=Join-Path $env:RUSTUP_HOME 'toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin'
    $env:PATH="$root\.tools\mingw\mingw64\bin;$toolBin;$toolBin\self-contained;$env:PATH"
}
Push-Location $root
try { & cargo @CargoArgs; $result=$LASTEXITCODE } finally {Pop-Location}
exit $result
